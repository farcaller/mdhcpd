#[macro_use(u32_bytes, bytes_u32)]
extern crate dhcp4r;
// extern crate etcd;

extern crate clap;

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate net2;
extern crate libc;

use std::net::UdpSocket;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::ops::Add;

use dhcp4r::{packet, options, server};

use clap::{Arg, App};

use std::os::unix::io::AsRawFd;
use std::ffi::CString;

// Server configuration
const SERVER_IP: [u8; 4] = [10, 99, 99, 1];
const IP_START: [u8; 4] = [10, 99, 99, 100];
const SUBNET_MASK: [u8; 4] = [255, 255, 255, 0];
const DNS_IPS: [u8; 8] = [8, 8, 8, 8, 8, 8, 4, 4]; // google dns servers
const ROUTER_IP: [u8; 4] = [192, 168, 0, 254];
const LEASE_DURATION_SECS: u32 = 30;
const LEASE_NUM: u32 = 100;

// Derrived constants
const LEASE_DURATION_BYTES: [u8; 4] = u32_bytes!(LEASE_DURATION_SECS);
const IP_START_NUM: u32 = bytes_u32!(IP_START);

fn main() {
    env_logger::init().unwrap();

    let matches = App::new("mdhcpd")
        .version("0.0.1")
        .arg(Arg::with_name("listen")
            .short("l")
            .long("listen")
            .required(true)
            .takes_value(true)
            .help("listen address and port")
            .default_value("0.0.0.0:67"))
        .arg(Arg::with_name("interface")
            .short("i")
            .long("interface")
            .required(true)
            .takes_value(true)
            .help("listen interface"))
        .arg(Arg::with_name("etcd_endpoints")
            .long("etcd_endpoints")
            .required(true)
            .takes_value(true)
            .help("etcd endpoints")
            .default_value("http://127.0.0.1:2379"))
        .get_matches();

    // let socket = net2::UdpBuilder::new_v4().unwrap();
    // let socket_fd = socket.as_raw_fd();
    // let iface_name_bytes = String::from(matches.value_of("interface").unwrap()).into_bytes();
    // let iface_name_len = iface_name_bytes.len();
    // let iface = CString::new(iface_name_bytes).unwrap();
    // unsafe {
    //     libc::setsockopt(socket_fd,
    //                      libc::SOL_SOCKET,
    //                      25, /*libc::SO_BINDTODEVICE*/
    //                      iface.as_ptr() as *const libc::c_void,
    //                      iface_name_len as u32)
    // };
    // let socket = socket.bind(matches.value_of("listen").unwrap());

    // // let socket = UdpSocket::bind(.expect("no listen value specified"));
    // if let Err(e) = socket {
    //     println!("{}", e);
    //     panic!();
    // }
    // let socket = socket.unwrap();
    // socket.set_broadcast(true).unwrap();

    let ms = MyServer {
        leases: HashMap::new(),
        last_lease: 0,
        lease_duration: Duration::new(LEASE_DURATION_SECS as u64, 0),
    };

    debug!("serving");
    // server::Server::serve(socket, SERVER_IP, ms);

    server::Server::bind_and_serve(Some(matches.value_of("interface").unwrap()), SERVER_IP, ms);
}

struct MyServer {
    leases: HashMap<u32, ([u8; 6], Instant)>,
    last_lease: u32,
    lease_duration: Duration,
}

impl server::Handler for MyServer {
    fn handle_request(&mut self,
                      server: &server::Server,
                      msg_type: u8,
                      in_packet: packet::Packet) {
        debug!("req {:?} {:?}", msg_type, in_packet.chaddr);
        match msg_type {
            dhcp4r::DISCOVER => {
                // Prefer client's choice if available
                if let Some(r) = in_packet.option(options::REQUESTED_IP_ADDRESS) {
                    if r.len() == 4 && self.available(&in_packet.chaddr, bytes_u32!(r)) {
                        reply(server, dhcp4r::OFFER, in_packet, [r[0], r[1], r[2], r[3]]);
                        return;
                    }
                }
                // Otherwise prefer existing (including expired if available)
                if let Some(ip) = self.current_lease(&in_packet.chaddr) {
                    reply(server, dhcp4r::OFFER, in_packet, u32_bytes!(ip));
                    return;
                }
                // Otherwise choose a free ip if available
                for _ in 0..LEASE_NUM {
                    self.last_lease = (self.last_lease + 1) % LEASE_NUM;
                    if self.available(&in_packet.chaddr, IP_START_NUM + &self.last_lease) {
                        reply(server,
                              dhcp4r::OFFER,
                              in_packet,
                              u32_bytes!(IP_START_NUM + &self.last_lease));
                        break;
                    }
                }
            }

            dhcp4r::REQUEST => {
                // Ignore requests to alternative DHCP server
                // if !server.for_this_server(&in_packet) {
                //     debug!("wrong server");
                //     return;
                // }
                let req_ip = match in_packet.option(options::REQUESTED_IP_ADDRESS) {
                    None => in_packet.ciaddr,
                    Some(x) => {
                        if x.len() != 4 {
                            debug!("bad ciaddr {:?}", x);
                            return;
                        } else {
                            [x[0], x[1], x[2], x[3]]
                        }
                    }
                };
                let req_ip_num = bytes_u32!(req_ip);
                if !&self.available(&in_packet.chaddr, req_ip_num) {
                    nak(server, in_packet, b"Requested IP not available");
                    debug!("not available {:?}", req_ip);
                    return;
                }
                self.leases.insert(req_ip_num,
                                   (in_packet.chaddr, Instant::now().add(self.lease_duration)));
                reply(server, dhcp4r::ACK, in_packet, req_ip);
            }

            dhcp4r::RELEASE | dhcp4r::DECLINE => {
                // Ignore requests to alternative DHCP server
                if !server.for_this_server(&in_packet) {
                    return;
                }
                if let Some(ip) = self.current_lease(&in_packet.chaddr) {
                    self.leases.remove(&ip);
                }
            }

            // TODO - not necessary but support for dhcp4r::INFORM might be nice
            _ => {}
        }
    }
}

impl MyServer {
    fn available(&self, chaddr: &[u8; 6], pos: u32) -> bool {
        debug!("avail {} {} {:?}",
               pos >= IP_START_NUM,
               pos < IP_START_NUM + LEASE_NUM,
               self.leases.get(&pos));
        match self.leases.get(&pos) {
            Some(x) => {
                debug!("chaddr {:?} = {:?}", *chaddr, x.0);
                let n = Instant::now();
                debug!("ttl expire {:?} now {:?} {} ", &x.1, n, x.1.gt(&n));
            }
            None => (),
        }
        return pos >= IP_START_NUM && pos < IP_START_NUM + LEASE_NUM &&
               match self.leases.get(&pos) {
            Some(x) => x.0 == *chaddr && x.1.gt(&Instant::now()),
            None => true,
        };
    }

    fn current_lease(&self, chaddr: &[u8; 6]) -> Option<u32> {
        for (i, v) in &self.leases {
            if &v.0 == chaddr {
                return Some(*i);
            }
        }
        return None;
    }
}

fn reply(s: &server::Server, msg_type: u8, req_packet: packet::Packet, offer_ip: [u8; 4]) {
    let _ = s.reply(msg_type,
                    vec![options::Option {
                             code: options::IP_ADDRESS_LEASE_TIME,
                             data: &LEASE_DURATION_BYTES,
                         },
                         options::Option {
                             code: options::SUBNET_MASK,
                             data: &SUBNET_MASK,
                         },
                         //  options::Option {
                         //      code: options::ROUTER,
                         //      data: &ROUTER_IP,
                         //  },
                         options::Option {
                             code: options::DOMAIN_NAME_SERVER,
                             data: &DNS_IPS,
                         }],
                    offer_ip,
                    req_packet);
}

fn nak(s: &server::Server, req_packet: packet::Packet, message: &[u8]) {
    let _ = s.reply(dhcp4r::NAK,
                    vec![options::Option {
                             code: options::MESSAGE,
                             data: message,
                         }],
                    [0, 0, 0, 0],
                    req_packet);
}
