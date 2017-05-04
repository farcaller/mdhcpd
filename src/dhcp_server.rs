use std::io;
use dhcp4r;
use dhcp4r::{packet, options, server};

pub struct DHCPServer {
    lease_duration: [u8; 4],
    subnet_mask: [u8; 4],
    router_ip: [u8; 4],
    dns_ips: [u8; 8],
}

impl DHCPServer {
    pub fn new() -> DHCPServer {
        DHCPServer {
            lease_duration: u32_bytes!(86_400 /* 24 hrs */),
            subnet_mask: [255, 255, 0, 0],
            router_ip: [10, 2, 0, 1],
            dns_ips: [8, 8, 8, 8, 8, 8, 4, 4],
        }
    }

    fn ip_for_mac(&self, mac: &[u8; 6]) -> Option<u32> {
        None
    }

    fn reply(&self, s: &server::Server, msg_type: u8, req_packet: packet::Packet, offer_ip: [u8; 4]) -> io::Result<()> {
        let opts = vec![options::Option {
                            code: options::IP_ADDRESS_LEASE_TIME,
                            data: &self.lease_duration,
                        },
                        options::Option {
                            code: options::SUBNET_MASK,
                            data: &self.subnet_mask,
                        },
                        options::Option {
                            code: options::ROUTER,
                            data: &self.router_ip,
                        },
                        options::Option {
                            code: options::DOMAIN_NAME_SERVER,
                            data: &self.dns_ips,
                        }];
        s.reply(msg_type, opts, offer_ip, req_packet).and_then(|_| Ok(()))
    }
}

impl server::Handler for DHCPServer {
    fn handle_request(&mut self, server: &server::Server, msg_type: u8, in_packet: packet::Packet) {
        match msg_type {
            dhcp4r::DISCOVER => {
                if let Some(ip) = self.ip_for_mac(&in_packet.chaddr) {
                    if let Err(e) = self.reply(server, dhcp4r::OFFER, in_packet, u32_bytes!(ip)) {
                        // TODO: implement
                    }
                }
            }

            dhcp4r::REQUEST => {
                if !server.for_this_server(&in_packet) {
                    return;
                }

                let req_ip = match in_packet.option(options::REQUESTED_IP_ADDRESS) {
                    None => in_packet.ciaddr,
                    Some(x) => {
                        if x.len() != 4 {
                            // TODO: log
                            return;
                        } else {
                            [x[0], x[1], x[2], x[3]]
                        }
                    }
                };
                let req_ip_num = bytes_u32!(req_ip);
                if !&self.available(&in_packet.chaddr, req_ip_num) {
                    nak(server, in_packet, b"Requested IP not available");
                    return;
                }
                self.leases.insert(req_ip_num, (in_packet.chaddr, Instant::now().add(self.lease_duration)));
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
