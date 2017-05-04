extern crate mdhcpd;

use std::net::Ipv4Addr;
use std::process::Command;

use mdhcpd::mac::MAC;
use mdhcpd::storage::EtcdClient;

fn run_etcd(args: &[&str]) -> String {
    let mut all_args = vec!["--endpoints=127.0.0.1:2379"];
    all_args.extend(args);
    let output = Command::new("etcdctl")
        .env("ETCDCTL_API", "3")
        .args(all_args)
        .output()
        .unwrap();
    let out = String::from_utf8(output.stdout).unwrap();
    println!("OUT: {} ERR: {}",
             out,
             String::from_utf8(output.stderr).unwrap());
    out
}

#[test]
fn add_etcd_key() {
    run_etcd(&["del", "--prefix", "/dhcp/"]);

    let etcd_cli = EtcdClient::new("127.0.0.1", 2379, "/dhcp/").unwrap();

    let ret = etcd_cli
        .try_add_hostname("test",
                          &"00:11:22:33:44:55".parse().unwrap(),
                          &"127.0.0.1".parse().unwrap())
        .unwrap();
    assert_eq!(ret, true);

    assert_eq!(run_etcd(&["get", "--prefix=true", "/dhcp/"]),
               "/dhcp/by-hostname/test/ip\n127.0.0.1\n/dhcp/by-hostname/test/mac\n00:11:22:33:44:\
                55\n/dhcp/by-ip/127.0.0.1/hostname\ntest\n/dhcp/by-ip/127.0.0.1/mac\n00:11:22:33:\
                44:55\n/dhcp/by-mac/00:11:22:33:44:55/hostname\ntest\n/dhcp/by-mac/00:11:22:33:\
                44:55/ip\n127.0.0.1\n");
}

#[test]
fn fail_to_add_dup_etcd_key() {
    run_etcd(&["del", "--prefix", "/dhcp/"]);

    let etcd_cli = EtcdClient::new("127.0.0.1", 2379, "/dhcp/").unwrap();

    let ret = etcd_cli
        .try_add_hostname("test",
                          &"00:11:22:33:44:55".parse().unwrap(),
                          &"127.0.0.1".parse().unwrap())
        .unwrap();
    assert_eq!(ret, true);

    let ret = etcd_cli
        .try_add_hostname("test",
                          &"00:11:22:33:44:55".parse().unwrap(),
                          &"127.0.0.1".parse().unwrap())
        .unwrap();
    assert_eq!(ret, false);
}

