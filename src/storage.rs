use std::net::Ipv4Addr;

use proto::rpc_grpc::{KV, KVClient};
use proto::rpc::{PutRequest, RangeRequest, Compare, TxnRequest, RequestOp, Compare_CompareResult,
                 Compare_CompareTarget};
use grpc::error::GrpcError;
use protobuf::RepeatedField;

use mac::MAC;

const BY_HOSTNAME_PREFIX: &'static str = "by-hostname/";
const BY_MAC_PREFIX: &'static str = "by-mac/";
const BY_IP_PREFIX: &'static str = "by-ip/";

fn put_op<T: Into<String>>(key: String, val: T) -> RequestOp {
    let mut req = PutRequest::new();
    req.set_key(key.into_bytes());
    req.set_value(val.into().into_bytes());
    let mut op = RequestOp::new();
    op.set_request_put(req);
    op
}

fn check_exists(key: String) -> Compare {
    let mut check = Compare::new();
    check.set_key(key.into_bytes());
    check.set_result(Compare_CompareResult::EQUAL);
    check.set_target(Compare_CompareTarget::CREATE);
    check.set_create_revision(0);
    check
}

pub struct EtcdClient {
    cli: KVClient,
    prefix: String,
}

impl EtcdClient {
    pub fn new<T: Into<String>>(host: &str, port: u16, prefix: T) -> Result<EtcdClient, GrpcError> {
        let cli = KVClient::new(host, port, false)?;
        Ok(EtcdClient {
               cli: cli,
               prefix: prefix.into(),
           })
    }

    pub fn try_add_hostname(&self,
                            hostname: &str,
                            mac: &MAC,
                            ip: &Ipv4Addr)
                            -> Result<bool, GrpcError> {
        let ip = format!("{}", ip);
        // PUT <HOSTNAME>/mac
        let key_host_mac = format!("{}{}{}/mac", self.prefix, BY_HOSTNAME_PREFIX, hostname);
        let put_host_mac_req_op = put_op(key_host_mac.clone(), mac.to_string());
        // PUT <HOSTNAME>/ip
        let key_host_ip = format!("{}{}{}/ip", self.prefix, BY_HOSTNAME_PREFIX, hostname);
        let put_host_ip_req_op = put_op(key_host_ip.clone(), ip.clone());
        // PUT <MAC>/hostname
        let key_mac_host = format!("{}{}{}/hostname", self.prefix, BY_MAC_PREFIX, mac);
        let put_mac_host_req_op = put_op(key_mac_host.clone(), hostname);
        // PUT <MAC>/ip
        let key_mac_ip = format!("{}{}{}/ip", self.prefix, BY_MAC_PREFIX, mac);
        let put_mac_ip_req_op = put_op(key_mac_ip.clone(), ip.clone());
        // PUT <IP>/mac
        let key_ip_mac = format!("{}{}{}/mac", self.prefix, BY_IP_PREFIX, ip);
        let put_ip_mac_req_op = put_op(key_ip_mac.clone(), mac.to_string());
        // PUT <IP>/hostname
        let key_ip_host = format!("{}{}{}/hostname", self.prefix, BY_IP_PREFIX, ip);
        let put_ip_host_req_op = put_op(key_ip_host.clone(), hostname);

        // CHECK <HOSTNAME>/mac
        let mut compare_host_req = Compare::new();
        compare_host_req.set_key(format!("{}{}{}/mac", self.prefix, BY_HOSTNAME_PREFIX, hostname)
                                     .into_bytes());
        compare_host_req.set_result(Compare_CompareResult::EQUAL);
        compare_host_req.set_target(Compare_CompareTarget::CREATE);
        compare_host_req.set_create_revision(0);

        let mut req = TxnRequest::new();
        req.set_success(RepeatedField::from_vec(vec![put_host_mac_req_op,
                                                     put_host_ip_req_op,
                                                     put_mac_host_req_op,
                                                     put_mac_ip_req_op,
                                                     put_ip_mac_req_op,
                                                     put_ip_host_req_op]));
        req.set_compare(RepeatedField::from_vec(vec![check_exists(key_host_mac),
                                                     check_exists(key_host_ip),
                                                     check_exists(key_mac_ip),
                                                     check_exists(key_mac_host),
                                                     check_exists(key_ip_mac),
                                                     check_exists(key_ip_host)]));
        let rep = self.cli.Txn(req)?;
        Ok(rep.succeeded)
    }
}

