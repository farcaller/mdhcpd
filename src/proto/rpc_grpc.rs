// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]


// interface

pub trait KV {
    fn Range(&self, p: super::rpc::RangeRequest) -> ::grpc::result::GrpcResult<super::rpc::RangeResponse>;

    fn Put(&self, p: super::rpc::PutRequest) -> ::grpc::result::GrpcResult<super::rpc::PutResponse>;

    fn DeleteRange(&self, p: super::rpc::DeleteRangeRequest) -> ::grpc::result::GrpcResult<super::rpc::DeleteRangeResponse>;

    fn Txn(&self, p: super::rpc::TxnRequest) -> ::grpc::result::GrpcResult<super::rpc::TxnResponse>;

    fn Compact(&self, p: super::rpc::CompactionRequest) -> ::grpc::result::GrpcResult<super::rpc::CompactionResponse>;
}

pub trait KVAsync {
    fn Range(&self, p: super::rpc::RangeRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::RangeResponse>;

    fn Put(&self, p: super::rpc::PutRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::PutResponse>;

    fn DeleteRange(&self, p: super::rpc::DeleteRangeRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::DeleteRangeResponse>;

    fn Txn(&self, p: super::rpc::TxnRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::TxnResponse>;

    fn Compact(&self, p: super::rpc::CompactionRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::CompactionResponse>;
}

// sync client

pub struct KVClient {
    async_client: KVAsyncClient,
}

impl KVClient {
    pub fn new(host: &str, port: u16, tls: bool) -> ::grpc::result::GrpcResult<Self> {
        KVAsyncClient::new(host, port, tls).map(|c| {
            KVClient {
                async_client: c,
            }
        })
    }
}

impl KV for KVClient {
    fn Range(&self, p: super::rpc::RangeRequest) -> ::grpc::result::GrpcResult<super::rpc::RangeResponse> {
        ::futures::Future::wait(self.async_client.Range(p))
    }

    fn Put(&self, p: super::rpc::PutRequest) -> ::grpc::result::GrpcResult<super::rpc::PutResponse> {
        ::futures::Future::wait(self.async_client.Put(p))
    }

    fn DeleteRange(&self, p: super::rpc::DeleteRangeRequest) -> ::grpc::result::GrpcResult<super::rpc::DeleteRangeResponse> {
        ::futures::Future::wait(self.async_client.DeleteRange(p))
    }

    fn Txn(&self, p: super::rpc::TxnRequest) -> ::grpc::result::GrpcResult<super::rpc::TxnResponse> {
        ::futures::Future::wait(self.async_client.Txn(p))
    }

    fn Compact(&self, p: super::rpc::CompactionRequest) -> ::grpc::result::GrpcResult<super::rpc::CompactionResponse> {
        ::futures::Future::wait(self.async_client.Compact(p))
    }
}

// async client

pub struct KVAsyncClient {
    grpc_client: ::grpc::client::GrpcClient,
    method_Range: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::rpc::RangeRequest, super::rpc::RangeResponse>>,
    method_Put: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::rpc::PutRequest, super::rpc::PutResponse>>,
    method_DeleteRange: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::rpc::DeleteRangeRequest, super::rpc::DeleteRangeResponse>>,
    method_Txn: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::rpc::TxnRequest, super::rpc::TxnResponse>>,
    method_Compact: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::rpc::CompactionRequest, super::rpc::CompactionResponse>>,
}

impl KVAsyncClient {
    pub fn new(host: &str, port: u16, tls: bool) -> ::grpc::result::GrpcResult<Self> {
        ::grpc::client::GrpcClient::new(host, port, tls).map(|c| {
            KVAsyncClient {
                grpc_client: c,
                method_Range: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/etcdserverpb.KV/Range".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_Put: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/etcdserverpb.KV/Put".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_DeleteRange: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/etcdserverpb.KV/DeleteRange".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_Txn: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/etcdserverpb.KV/Txn".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_Compact: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/etcdserverpb.KV/Compact".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
            }
        })
    }
}

impl KVAsync for KVAsyncClient {
    fn Range(&self, p: super::rpc::RangeRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::RangeResponse> {
        self.grpc_client.call_unary(p, self.method_Range.clone())
    }

    fn Put(&self, p: super::rpc::PutRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::PutResponse> {
        self.grpc_client.call_unary(p, self.method_Put.clone())
    }

    fn DeleteRange(&self, p: super::rpc::DeleteRangeRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::DeleteRangeResponse> {
        self.grpc_client.call_unary(p, self.method_DeleteRange.clone())
    }

    fn Txn(&self, p: super::rpc::TxnRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::TxnResponse> {
        self.grpc_client.call_unary(p, self.method_Txn.clone())
    }

    fn Compact(&self, p: super::rpc::CompactionRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::CompactionResponse> {
        self.grpc_client.call_unary(p, self.method_Compact.clone())
    }
}

// sync server

pub struct KVServer {
    async_server: KVAsyncServer,
}

struct KVServerHandlerToAsync {
    handler: ::std::sync::Arc<KV + Send + Sync>,
    cpupool: ::futures_cpupool::CpuPool,
}

impl KVAsync for KVServerHandlerToAsync {
    fn Range(&self, p: super::rpc::RangeRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::RangeResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.Range(p)
        })
    }

    fn Put(&self, p: super::rpc::PutRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::PutResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.Put(p)
        })
    }

    fn DeleteRange(&self, p: super::rpc::DeleteRangeRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::DeleteRangeResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.DeleteRange(p)
        })
    }

    fn Txn(&self, p: super::rpc::TxnRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::TxnResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.Txn(p)
        })
    }

    fn Compact(&self, p: super::rpc::CompactionRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::CompactionResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.Compact(p)
        })
    }
}

impl KVServer {
    pub fn new<A : ::std::net::ToSocketAddrs, H : KV + Send + Sync + 'static>(addr: A, h: H) -> Self {
        let h = KVServerHandlerToAsync {
            cpupool: ::futures_cpupool::CpuPool::new_num_cpus(),
            handler: ::std::sync::Arc::new(h),
        };
        KVServer {
            async_server: KVAsyncServer::new(addr, h),
        }
    }
}

// async server

pub struct KVAsyncServer {
    grpc_server: ::grpc::server::GrpcServer,
}

impl KVAsyncServer {
    pub fn new<A : ::std::net::ToSocketAddrs, H : KVAsync + 'static + Sync + Send + 'static>(addr: A, h: H) -> Self {
        let service_definition = KVAsyncServer::new_service_def(h);
        KVAsyncServer {
            grpc_server: ::grpc::server::GrpcServer::new(addr, service_definition),
        }
    }

    pub fn new_service_def<H : KVAsync + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::server::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::server::ServerServiceDefinition::new(
            vec![
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/etcdserverpb.KV/Range".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.Range(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/etcdserverpb.KV/Put".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.Put(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/etcdserverpb.KV/DeleteRange".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.DeleteRange(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/etcdserverpb.KV/Txn".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.Txn(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/etcdserverpb.KV/Compact".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.Compact(p))
                    },
                ),
            ],
        )
    }
}

// interface

pub trait Watch {
    fn Watch(&self, p: ::grpc::iter::GrpcIterator<super::rpc::WatchRequest>) -> ::grpc::iter::GrpcIterator<super::rpc::WatchResponse>;
}

pub trait WatchAsync {
    fn Watch(&self, p: ::grpc::futures_grpc::GrpcStreamSend<super::rpc::WatchRequest>) -> ::grpc::futures_grpc::GrpcStreamSend<super::rpc::WatchResponse>;
}

// sync client

pub struct WatchClient {
    async_client: WatchAsyncClient,
}

impl WatchClient {
    pub fn new(host: &str, port: u16, tls: bool) -> ::grpc::result::GrpcResult<Self> {
        WatchAsyncClient::new(host, port, tls).map(|c| {
            WatchClient {
                async_client: c,
            }
        })
    }
}

impl Watch for WatchClient {
    fn Watch(&self, p: ::grpc::iter::GrpcIterator<super::rpc::WatchRequest>) -> ::grpc::iter::GrpcIterator<super::rpc::WatchResponse> {
        let p = ::futures::stream::Stream::boxed(::futures::stream::iter(::std::iter::IntoIterator::into_iter(p)));
        ::grpc::rt::stream_to_iter(self.async_client.Watch(p))
    }
}

// async client

pub struct WatchAsyncClient {
    grpc_client: ::grpc::client::GrpcClient,
    method_Watch: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::rpc::WatchRequest, super::rpc::WatchResponse>>,
}

impl WatchAsyncClient {
    pub fn new(host: &str, port: u16, tls: bool) -> ::grpc::result::GrpcResult<Self> {
        ::grpc::client::GrpcClient::new(host, port, tls).map(|c| {
            WatchAsyncClient {
                grpc_client: c,
                method_Watch: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/etcdserverpb.Watch/Watch".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Bidi,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
            }
        })
    }
}

impl WatchAsync for WatchAsyncClient {
    fn Watch(&self, p: ::grpc::futures_grpc::GrpcStreamSend<super::rpc::WatchRequest>) -> ::grpc::futures_grpc::GrpcStreamSend<super::rpc::WatchResponse> {
        self.grpc_client.call_bidi(p, self.method_Watch.clone())
    }
}

// sync server

pub struct WatchServer {
    async_server: WatchAsyncServer,
}

struct WatchServerHandlerToAsync {
    handler: ::std::sync::Arc<Watch + Send + Sync>,
    cpupool: ::futures_cpupool::CpuPool,
}

impl WatchAsync for WatchServerHandlerToAsync {
    fn Watch(&self, p: ::grpc::futures_grpc::GrpcStreamSend<super::rpc::WatchRequest>) -> ::grpc::futures_grpc::GrpcStreamSend<super::rpc::WatchResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_bidi(&self.cpupool, p, move |p| {
            h.Watch(p)
        })
    }
}

impl WatchServer {
    pub fn new<A : ::std::net::ToSocketAddrs, H : Watch + Send + Sync + 'static>(addr: A, h: H) -> Self {
        let h = WatchServerHandlerToAsync {
            cpupool: ::futures_cpupool::CpuPool::new_num_cpus(),
            handler: ::std::sync::Arc::new(h),
        };
        WatchServer {
            async_server: WatchAsyncServer::new(addr, h),
        }
    }
}

// async server

pub struct WatchAsyncServer {
    grpc_server: ::grpc::server::GrpcServer,
}

impl WatchAsyncServer {
    pub fn new<A : ::std::net::ToSocketAddrs, H : WatchAsync + 'static + Sync + Send + 'static>(addr: A, h: H) -> Self {
        let service_definition = WatchAsyncServer::new_service_def(h);
        WatchAsyncServer {
            grpc_server: ::grpc::server::GrpcServer::new(addr, service_definition),
        }
    }

    pub fn new_service_def<H : WatchAsync + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::server::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::server::ServerServiceDefinition::new(
            vec![
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/etcdserverpb.Watch/Watch".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Bidi,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerBidi::new(move |p| handler_copy.Watch(p))
                    },
                ),
            ],
        )
    }
}

// interface

pub trait Lease {
    fn LeaseGrant(&self, p: super::rpc::LeaseGrantRequest) -> ::grpc::result::GrpcResult<super::rpc::LeaseGrantResponse>;

    fn LeaseRevoke(&self, p: super::rpc::LeaseRevokeRequest) -> ::grpc::result::GrpcResult<super::rpc::LeaseRevokeResponse>;

    fn LeaseKeepAlive(&self, p: ::grpc::iter::GrpcIterator<super::rpc::LeaseKeepAliveRequest>) -> ::grpc::iter::GrpcIterator<super::rpc::LeaseKeepAliveResponse>;

    fn LeaseTimeToLive(&self, p: super::rpc::LeaseTimeToLiveRequest) -> ::grpc::result::GrpcResult<super::rpc::LeaseTimeToLiveResponse>;
}

pub trait LeaseAsync {
    fn LeaseGrant(&self, p: super::rpc::LeaseGrantRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::LeaseGrantResponse>;

    fn LeaseRevoke(&self, p: super::rpc::LeaseRevokeRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::LeaseRevokeResponse>;

    fn LeaseKeepAlive(&self, p: ::grpc::futures_grpc::GrpcStreamSend<super::rpc::LeaseKeepAliveRequest>) -> ::grpc::futures_grpc::GrpcStreamSend<super::rpc::LeaseKeepAliveResponse>;

    fn LeaseTimeToLive(&self, p: super::rpc::LeaseTimeToLiveRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::LeaseTimeToLiveResponse>;
}

// sync client

pub struct LeaseClient {
    async_client: LeaseAsyncClient,
}

impl LeaseClient {
    pub fn new(host: &str, port: u16, tls: bool) -> ::grpc::result::GrpcResult<Self> {
        LeaseAsyncClient::new(host, port, tls).map(|c| {
            LeaseClient {
                async_client: c,
            }
        })
    }
}

impl Lease for LeaseClient {
    fn LeaseGrant(&self, p: super::rpc::LeaseGrantRequest) -> ::grpc::result::GrpcResult<super::rpc::LeaseGrantResponse> {
        ::futures::Future::wait(self.async_client.LeaseGrant(p))
    }

    fn LeaseRevoke(&self, p: super::rpc::LeaseRevokeRequest) -> ::grpc::result::GrpcResult<super::rpc::LeaseRevokeResponse> {
        ::futures::Future::wait(self.async_client.LeaseRevoke(p))
    }

    fn LeaseKeepAlive(&self, p: ::grpc::iter::GrpcIterator<super::rpc::LeaseKeepAliveRequest>) -> ::grpc::iter::GrpcIterator<super::rpc::LeaseKeepAliveResponse> {
        let p = ::futures::stream::Stream::boxed(::futures::stream::iter(::std::iter::IntoIterator::into_iter(p)));
        ::grpc::rt::stream_to_iter(self.async_client.LeaseKeepAlive(p))
    }

    fn LeaseTimeToLive(&self, p: super::rpc::LeaseTimeToLiveRequest) -> ::grpc::result::GrpcResult<super::rpc::LeaseTimeToLiveResponse> {
        ::futures::Future::wait(self.async_client.LeaseTimeToLive(p))
    }
}

// async client

pub struct LeaseAsyncClient {
    grpc_client: ::grpc::client::GrpcClient,
    method_LeaseGrant: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::rpc::LeaseGrantRequest, super::rpc::LeaseGrantResponse>>,
    method_LeaseRevoke: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::rpc::LeaseRevokeRequest, super::rpc::LeaseRevokeResponse>>,
    method_LeaseKeepAlive: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::rpc::LeaseKeepAliveRequest, super::rpc::LeaseKeepAliveResponse>>,
    method_LeaseTimeToLive: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::rpc::LeaseTimeToLiveRequest, super::rpc::LeaseTimeToLiveResponse>>,
}

impl LeaseAsyncClient {
    pub fn new(host: &str, port: u16, tls: bool) -> ::grpc::result::GrpcResult<Self> {
        ::grpc::client::GrpcClient::new(host, port, tls).map(|c| {
            LeaseAsyncClient {
                grpc_client: c,
                method_LeaseGrant: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/etcdserverpb.Lease/LeaseGrant".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_LeaseRevoke: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/etcdserverpb.Lease/LeaseRevoke".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_LeaseKeepAlive: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/etcdserverpb.Lease/LeaseKeepAlive".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Bidi,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_LeaseTimeToLive: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/etcdserverpb.Lease/LeaseTimeToLive".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
            }
        })
    }
}

impl LeaseAsync for LeaseAsyncClient {
    fn LeaseGrant(&self, p: super::rpc::LeaseGrantRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::LeaseGrantResponse> {
        self.grpc_client.call_unary(p, self.method_LeaseGrant.clone())
    }

    fn LeaseRevoke(&self, p: super::rpc::LeaseRevokeRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::LeaseRevokeResponse> {
        self.grpc_client.call_unary(p, self.method_LeaseRevoke.clone())
    }

    fn LeaseKeepAlive(&self, p: ::grpc::futures_grpc::GrpcStreamSend<super::rpc::LeaseKeepAliveRequest>) -> ::grpc::futures_grpc::GrpcStreamSend<super::rpc::LeaseKeepAliveResponse> {
        self.grpc_client.call_bidi(p, self.method_LeaseKeepAlive.clone())
    }

    fn LeaseTimeToLive(&self, p: super::rpc::LeaseTimeToLiveRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::LeaseTimeToLiveResponse> {
        self.grpc_client.call_unary(p, self.method_LeaseTimeToLive.clone())
    }
}

// sync server

pub struct LeaseServer {
    async_server: LeaseAsyncServer,
}

struct LeaseServerHandlerToAsync {
    handler: ::std::sync::Arc<Lease + Send + Sync>,
    cpupool: ::futures_cpupool::CpuPool,
}

impl LeaseAsync for LeaseServerHandlerToAsync {
    fn LeaseGrant(&self, p: super::rpc::LeaseGrantRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::LeaseGrantResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.LeaseGrant(p)
        })
    }

    fn LeaseRevoke(&self, p: super::rpc::LeaseRevokeRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::LeaseRevokeResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.LeaseRevoke(p)
        })
    }

    fn LeaseKeepAlive(&self, p: ::grpc::futures_grpc::GrpcStreamSend<super::rpc::LeaseKeepAliveRequest>) -> ::grpc::futures_grpc::GrpcStreamSend<super::rpc::LeaseKeepAliveResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_bidi(&self.cpupool, p, move |p| {
            h.LeaseKeepAlive(p)
        })
    }

    fn LeaseTimeToLive(&self, p: super::rpc::LeaseTimeToLiveRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::LeaseTimeToLiveResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.LeaseTimeToLive(p)
        })
    }
}

impl LeaseServer {
    pub fn new<A : ::std::net::ToSocketAddrs, H : Lease + Send + Sync + 'static>(addr: A, h: H) -> Self {
        let h = LeaseServerHandlerToAsync {
            cpupool: ::futures_cpupool::CpuPool::new_num_cpus(),
            handler: ::std::sync::Arc::new(h),
        };
        LeaseServer {
            async_server: LeaseAsyncServer::new(addr, h),
        }
    }
}

// async server

pub struct LeaseAsyncServer {
    grpc_server: ::grpc::server::GrpcServer,
}

impl LeaseAsyncServer {
    pub fn new<A : ::std::net::ToSocketAddrs, H : LeaseAsync + 'static + Sync + Send + 'static>(addr: A, h: H) -> Self {
        let service_definition = LeaseAsyncServer::new_service_def(h);
        LeaseAsyncServer {
            grpc_server: ::grpc::server::GrpcServer::new(addr, service_definition),
        }
    }

    pub fn new_service_def<H : LeaseAsync + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::server::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::server::ServerServiceDefinition::new(
            vec![
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/etcdserverpb.Lease/LeaseGrant".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.LeaseGrant(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/etcdserverpb.Lease/LeaseRevoke".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.LeaseRevoke(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/etcdserverpb.Lease/LeaseKeepAlive".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Bidi,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerBidi::new(move |p| handler_copy.LeaseKeepAlive(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/etcdserverpb.Lease/LeaseTimeToLive".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.LeaseTimeToLive(p))
                    },
                ),
            ],
        )
    }
}

// interface

pub trait Cluster {
    fn MemberAdd(&self, p: super::rpc::MemberAddRequest) -> ::grpc::result::GrpcResult<super::rpc::MemberAddResponse>;

    fn MemberRemove(&self, p: super::rpc::MemberRemoveRequest) -> ::grpc::result::GrpcResult<super::rpc::MemberRemoveResponse>;

    fn MemberUpdate(&self, p: super::rpc::MemberUpdateRequest) -> ::grpc::result::GrpcResult<super::rpc::MemberUpdateResponse>;

    fn MemberList(&self, p: super::rpc::MemberListRequest) -> ::grpc::result::GrpcResult<super::rpc::MemberListResponse>;
}

pub trait ClusterAsync {
    fn MemberAdd(&self, p: super::rpc::MemberAddRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::MemberAddResponse>;

    fn MemberRemove(&self, p: super::rpc::MemberRemoveRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::MemberRemoveResponse>;

    fn MemberUpdate(&self, p: super::rpc::MemberUpdateRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::MemberUpdateResponse>;

    fn MemberList(&self, p: super::rpc::MemberListRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::MemberListResponse>;
}

// sync client

pub struct ClusterClient {
    async_client: ClusterAsyncClient,
}

impl ClusterClient {
    pub fn new(host: &str, port: u16, tls: bool) -> ::grpc::result::GrpcResult<Self> {
        ClusterAsyncClient::new(host, port, tls).map(|c| {
            ClusterClient {
                async_client: c,
            }
        })
    }
}

impl Cluster for ClusterClient {
    fn MemberAdd(&self, p: super::rpc::MemberAddRequest) -> ::grpc::result::GrpcResult<super::rpc::MemberAddResponse> {
        ::futures::Future::wait(self.async_client.MemberAdd(p))
    }

    fn MemberRemove(&self, p: super::rpc::MemberRemoveRequest) -> ::grpc::result::GrpcResult<super::rpc::MemberRemoveResponse> {
        ::futures::Future::wait(self.async_client.MemberRemove(p))
    }

    fn MemberUpdate(&self, p: super::rpc::MemberUpdateRequest) -> ::grpc::result::GrpcResult<super::rpc::MemberUpdateResponse> {
        ::futures::Future::wait(self.async_client.MemberUpdate(p))
    }

    fn MemberList(&self, p: super::rpc::MemberListRequest) -> ::grpc::result::GrpcResult<super::rpc::MemberListResponse> {
        ::futures::Future::wait(self.async_client.MemberList(p))
    }
}

// async client

pub struct ClusterAsyncClient {
    grpc_client: ::grpc::client::GrpcClient,
    method_MemberAdd: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::rpc::MemberAddRequest, super::rpc::MemberAddResponse>>,
    method_MemberRemove: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::rpc::MemberRemoveRequest, super::rpc::MemberRemoveResponse>>,
    method_MemberUpdate: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::rpc::MemberUpdateRequest, super::rpc::MemberUpdateResponse>>,
    method_MemberList: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::rpc::MemberListRequest, super::rpc::MemberListResponse>>,
}

impl ClusterAsyncClient {
    pub fn new(host: &str, port: u16, tls: bool) -> ::grpc::result::GrpcResult<Self> {
        ::grpc::client::GrpcClient::new(host, port, tls).map(|c| {
            ClusterAsyncClient {
                grpc_client: c,
                method_MemberAdd: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/etcdserverpb.Cluster/MemberAdd".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_MemberRemove: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/etcdserverpb.Cluster/MemberRemove".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_MemberUpdate: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/etcdserverpb.Cluster/MemberUpdate".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_MemberList: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/etcdserverpb.Cluster/MemberList".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
            }
        })
    }
}

impl ClusterAsync for ClusterAsyncClient {
    fn MemberAdd(&self, p: super::rpc::MemberAddRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::MemberAddResponse> {
        self.grpc_client.call_unary(p, self.method_MemberAdd.clone())
    }

    fn MemberRemove(&self, p: super::rpc::MemberRemoveRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::MemberRemoveResponse> {
        self.grpc_client.call_unary(p, self.method_MemberRemove.clone())
    }

    fn MemberUpdate(&self, p: super::rpc::MemberUpdateRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::MemberUpdateResponse> {
        self.grpc_client.call_unary(p, self.method_MemberUpdate.clone())
    }

    fn MemberList(&self, p: super::rpc::MemberListRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::MemberListResponse> {
        self.grpc_client.call_unary(p, self.method_MemberList.clone())
    }
}

// sync server

pub struct ClusterServer {
    async_server: ClusterAsyncServer,
}

struct ClusterServerHandlerToAsync {
    handler: ::std::sync::Arc<Cluster + Send + Sync>,
    cpupool: ::futures_cpupool::CpuPool,
}

impl ClusterAsync for ClusterServerHandlerToAsync {
    fn MemberAdd(&self, p: super::rpc::MemberAddRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::MemberAddResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.MemberAdd(p)
        })
    }

    fn MemberRemove(&self, p: super::rpc::MemberRemoveRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::MemberRemoveResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.MemberRemove(p)
        })
    }

    fn MemberUpdate(&self, p: super::rpc::MemberUpdateRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::MemberUpdateResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.MemberUpdate(p)
        })
    }

    fn MemberList(&self, p: super::rpc::MemberListRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::MemberListResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.MemberList(p)
        })
    }
}

impl ClusterServer {
    pub fn new<A : ::std::net::ToSocketAddrs, H : Cluster + Send + Sync + 'static>(addr: A, h: H) -> Self {
        let h = ClusterServerHandlerToAsync {
            cpupool: ::futures_cpupool::CpuPool::new_num_cpus(),
            handler: ::std::sync::Arc::new(h),
        };
        ClusterServer {
            async_server: ClusterAsyncServer::new(addr, h),
        }
    }
}

// async server

pub struct ClusterAsyncServer {
    grpc_server: ::grpc::server::GrpcServer,
}

impl ClusterAsyncServer {
    pub fn new<A : ::std::net::ToSocketAddrs, H : ClusterAsync + 'static + Sync + Send + 'static>(addr: A, h: H) -> Self {
        let service_definition = ClusterAsyncServer::new_service_def(h);
        ClusterAsyncServer {
            grpc_server: ::grpc::server::GrpcServer::new(addr, service_definition),
        }
    }

    pub fn new_service_def<H : ClusterAsync + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::server::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::server::ServerServiceDefinition::new(
            vec![
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/etcdserverpb.Cluster/MemberAdd".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.MemberAdd(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/etcdserverpb.Cluster/MemberRemove".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.MemberRemove(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/etcdserverpb.Cluster/MemberUpdate".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.MemberUpdate(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/etcdserverpb.Cluster/MemberList".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.MemberList(p))
                    },
                ),
            ],
        )
    }
}

// interface

pub trait Maintenance {
    fn Alarm(&self, p: super::rpc::AlarmRequest) -> ::grpc::result::GrpcResult<super::rpc::AlarmResponse>;

    fn Status(&self, p: super::rpc::StatusRequest) -> ::grpc::result::GrpcResult<super::rpc::StatusResponse>;

    fn Defragment(&self, p: super::rpc::DefragmentRequest) -> ::grpc::result::GrpcResult<super::rpc::DefragmentResponse>;

    fn Hash(&self, p: super::rpc::HashRequest) -> ::grpc::result::GrpcResult<super::rpc::HashResponse>;

    fn Snapshot(&self, p: super::rpc::SnapshotRequest) -> ::grpc::iter::GrpcIterator<super::rpc::SnapshotResponse>;
}

pub trait MaintenanceAsync {
    fn Alarm(&self, p: super::rpc::AlarmRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::AlarmResponse>;

    fn Status(&self, p: super::rpc::StatusRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::StatusResponse>;

    fn Defragment(&self, p: super::rpc::DefragmentRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::DefragmentResponse>;

    fn Hash(&self, p: super::rpc::HashRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::HashResponse>;

    fn Snapshot(&self, p: super::rpc::SnapshotRequest) -> ::grpc::futures_grpc::GrpcStreamSend<super::rpc::SnapshotResponse>;
}

// sync client

pub struct MaintenanceClient {
    async_client: MaintenanceAsyncClient,
}

impl MaintenanceClient {
    pub fn new(host: &str, port: u16, tls: bool) -> ::grpc::result::GrpcResult<Self> {
        MaintenanceAsyncClient::new(host, port, tls).map(|c| {
            MaintenanceClient {
                async_client: c,
            }
        })
    }
}

impl Maintenance for MaintenanceClient {
    fn Alarm(&self, p: super::rpc::AlarmRequest) -> ::grpc::result::GrpcResult<super::rpc::AlarmResponse> {
        ::futures::Future::wait(self.async_client.Alarm(p))
    }

    fn Status(&self, p: super::rpc::StatusRequest) -> ::grpc::result::GrpcResult<super::rpc::StatusResponse> {
        ::futures::Future::wait(self.async_client.Status(p))
    }

    fn Defragment(&self, p: super::rpc::DefragmentRequest) -> ::grpc::result::GrpcResult<super::rpc::DefragmentResponse> {
        ::futures::Future::wait(self.async_client.Defragment(p))
    }

    fn Hash(&self, p: super::rpc::HashRequest) -> ::grpc::result::GrpcResult<super::rpc::HashResponse> {
        ::futures::Future::wait(self.async_client.Hash(p))
    }

    fn Snapshot(&self, p: super::rpc::SnapshotRequest) -> ::grpc::iter::GrpcIterator<super::rpc::SnapshotResponse> {
        ::grpc::rt::stream_to_iter(self.async_client.Snapshot(p))
    }
}

// async client

pub struct MaintenanceAsyncClient {
    grpc_client: ::grpc::client::GrpcClient,
    method_Alarm: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::rpc::AlarmRequest, super::rpc::AlarmResponse>>,
    method_Status: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::rpc::StatusRequest, super::rpc::StatusResponse>>,
    method_Defragment: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::rpc::DefragmentRequest, super::rpc::DefragmentResponse>>,
    method_Hash: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::rpc::HashRequest, super::rpc::HashResponse>>,
    method_Snapshot: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::rpc::SnapshotRequest, super::rpc::SnapshotResponse>>,
}

impl MaintenanceAsyncClient {
    pub fn new(host: &str, port: u16, tls: bool) -> ::grpc::result::GrpcResult<Self> {
        ::grpc::client::GrpcClient::new(host, port, tls).map(|c| {
            MaintenanceAsyncClient {
                grpc_client: c,
                method_Alarm: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/etcdserverpb.Maintenance/Alarm".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_Status: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/etcdserverpb.Maintenance/Status".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_Defragment: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/etcdserverpb.Maintenance/Defragment".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_Hash: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/etcdserverpb.Maintenance/Hash".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_Snapshot: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/etcdserverpb.Maintenance/Snapshot".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::ServerStreaming,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
            }
        })
    }
}

impl MaintenanceAsync for MaintenanceAsyncClient {
    fn Alarm(&self, p: super::rpc::AlarmRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::AlarmResponse> {
        self.grpc_client.call_unary(p, self.method_Alarm.clone())
    }

    fn Status(&self, p: super::rpc::StatusRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::StatusResponse> {
        self.grpc_client.call_unary(p, self.method_Status.clone())
    }

    fn Defragment(&self, p: super::rpc::DefragmentRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::DefragmentResponse> {
        self.grpc_client.call_unary(p, self.method_Defragment.clone())
    }

    fn Hash(&self, p: super::rpc::HashRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::HashResponse> {
        self.grpc_client.call_unary(p, self.method_Hash.clone())
    }

    fn Snapshot(&self, p: super::rpc::SnapshotRequest) -> ::grpc::futures_grpc::GrpcStreamSend<super::rpc::SnapshotResponse> {
        self.grpc_client.call_server_streaming(p, self.method_Snapshot.clone())
    }
}

// sync server

pub struct MaintenanceServer {
    async_server: MaintenanceAsyncServer,
}

struct MaintenanceServerHandlerToAsync {
    handler: ::std::sync::Arc<Maintenance + Send + Sync>,
    cpupool: ::futures_cpupool::CpuPool,
}

impl MaintenanceAsync for MaintenanceServerHandlerToAsync {
    fn Alarm(&self, p: super::rpc::AlarmRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::AlarmResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.Alarm(p)
        })
    }

    fn Status(&self, p: super::rpc::StatusRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::StatusResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.Status(p)
        })
    }

    fn Defragment(&self, p: super::rpc::DefragmentRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::DefragmentResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.Defragment(p)
        })
    }

    fn Hash(&self, p: super::rpc::HashRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::HashResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.Hash(p)
        })
    }

    fn Snapshot(&self, p: super::rpc::SnapshotRequest) -> ::grpc::futures_grpc::GrpcStreamSend<super::rpc::SnapshotResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_server_streaming(&self.cpupool, p, move |p| {
            h.Snapshot(p)
        })
    }
}

impl MaintenanceServer {
    pub fn new<A : ::std::net::ToSocketAddrs, H : Maintenance + Send + Sync + 'static>(addr: A, h: H) -> Self {
        let h = MaintenanceServerHandlerToAsync {
            cpupool: ::futures_cpupool::CpuPool::new_num_cpus(),
            handler: ::std::sync::Arc::new(h),
        };
        MaintenanceServer {
            async_server: MaintenanceAsyncServer::new(addr, h),
        }
    }
}

// async server

pub struct MaintenanceAsyncServer {
    grpc_server: ::grpc::server::GrpcServer,
}

impl MaintenanceAsyncServer {
    pub fn new<A : ::std::net::ToSocketAddrs, H : MaintenanceAsync + 'static + Sync + Send + 'static>(addr: A, h: H) -> Self {
        let service_definition = MaintenanceAsyncServer::new_service_def(h);
        MaintenanceAsyncServer {
            grpc_server: ::grpc::server::GrpcServer::new(addr, service_definition),
        }
    }

    pub fn new_service_def<H : MaintenanceAsync + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::server::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::server::ServerServiceDefinition::new(
            vec![
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/etcdserverpb.Maintenance/Alarm".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.Alarm(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/etcdserverpb.Maintenance/Status".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.Status(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/etcdserverpb.Maintenance/Defragment".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.Defragment(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/etcdserverpb.Maintenance/Hash".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.Hash(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/etcdserverpb.Maintenance/Snapshot".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::ServerStreaming,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerServerStreaming::new(move |p| handler_copy.Snapshot(p))
                    },
                ),
            ],
        )
    }
}

// interface

pub trait Auth {
    fn AuthEnable(&self, p: super::rpc::AuthEnableRequest) -> ::grpc::result::GrpcResult<super::rpc::AuthEnableResponse>;

    fn AuthDisable(&self, p: super::rpc::AuthDisableRequest) -> ::grpc::result::GrpcResult<super::rpc::AuthDisableResponse>;

    fn Authenticate(&self, p: super::rpc::AuthenticateRequest) -> ::grpc::result::GrpcResult<super::rpc::AuthenticateResponse>;

    fn UserAdd(&self, p: super::rpc::AuthUserAddRequest) -> ::grpc::result::GrpcResult<super::rpc::AuthUserAddResponse>;

    fn UserGet(&self, p: super::rpc::AuthUserGetRequest) -> ::grpc::result::GrpcResult<super::rpc::AuthUserGetResponse>;

    fn UserList(&self, p: super::rpc::AuthUserListRequest) -> ::grpc::result::GrpcResult<super::rpc::AuthUserListResponse>;

    fn UserDelete(&self, p: super::rpc::AuthUserDeleteRequest) -> ::grpc::result::GrpcResult<super::rpc::AuthUserDeleteResponse>;

    fn UserChangePassword(&self, p: super::rpc::AuthUserChangePasswordRequest) -> ::grpc::result::GrpcResult<super::rpc::AuthUserChangePasswordResponse>;

    fn UserGrantRole(&self, p: super::rpc::AuthUserGrantRoleRequest) -> ::grpc::result::GrpcResult<super::rpc::AuthUserGrantRoleResponse>;

    fn UserRevokeRole(&self, p: super::rpc::AuthUserRevokeRoleRequest) -> ::grpc::result::GrpcResult<super::rpc::AuthUserRevokeRoleResponse>;

    fn RoleAdd(&self, p: super::rpc::AuthRoleAddRequest) -> ::grpc::result::GrpcResult<super::rpc::AuthRoleAddResponse>;

    fn RoleGet(&self, p: super::rpc::AuthRoleGetRequest) -> ::grpc::result::GrpcResult<super::rpc::AuthRoleGetResponse>;

    fn RoleList(&self, p: super::rpc::AuthRoleListRequest) -> ::grpc::result::GrpcResult<super::rpc::AuthRoleListResponse>;

    fn RoleDelete(&self, p: super::rpc::AuthRoleDeleteRequest) -> ::grpc::result::GrpcResult<super::rpc::AuthRoleDeleteResponse>;

    fn RoleGrantPermission(&self, p: super::rpc::AuthRoleGrantPermissionRequest) -> ::grpc::result::GrpcResult<super::rpc::AuthRoleGrantPermissionResponse>;

    fn RoleRevokePermission(&self, p: super::rpc::AuthRoleRevokePermissionRequest) -> ::grpc::result::GrpcResult<super::rpc::AuthRoleRevokePermissionResponse>;
}

pub trait AuthAsync {
    fn AuthEnable(&self, p: super::rpc::AuthEnableRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::AuthEnableResponse>;

    fn AuthDisable(&self, p: super::rpc::AuthDisableRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::AuthDisableResponse>;

    fn Authenticate(&self, p: super::rpc::AuthenticateRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::AuthenticateResponse>;

    fn UserAdd(&self, p: super::rpc::AuthUserAddRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::AuthUserAddResponse>;

    fn UserGet(&self, p: super::rpc::AuthUserGetRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::AuthUserGetResponse>;

    fn UserList(&self, p: super::rpc::AuthUserListRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::AuthUserListResponse>;

    fn UserDelete(&self, p: super::rpc::AuthUserDeleteRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::AuthUserDeleteResponse>;

    fn UserChangePassword(&self, p: super::rpc::AuthUserChangePasswordRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::AuthUserChangePasswordResponse>;

    fn UserGrantRole(&self, p: super::rpc::AuthUserGrantRoleRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::AuthUserGrantRoleResponse>;

    fn UserRevokeRole(&self, p: super::rpc::AuthUserRevokeRoleRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::AuthUserRevokeRoleResponse>;

    fn RoleAdd(&self, p: super::rpc::AuthRoleAddRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::AuthRoleAddResponse>;

    fn RoleGet(&self, p: super::rpc::AuthRoleGetRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::AuthRoleGetResponse>;

    fn RoleList(&self, p: super::rpc::AuthRoleListRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::AuthRoleListResponse>;

    fn RoleDelete(&self, p: super::rpc::AuthRoleDeleteRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::AuthRoleDeleteResponse>;

    fn RoleGrantPermission(&self, p: super::rpc::AuthRoleGrantPermissionRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::AuthRoleGrantPermissionResponse>;

    fn RoleRevokePermission(&self, p: super::rpc::AuthRoleRevokePermissionRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::AuthRoleRevokePermissionResponse>;
}

// sync client

pub struct AuthClient {
    async_client: AuthAsyncClient,
}

impl AuthClient {
    pub fn new(host: &str, port: u16, tls: bool) -> ::grpc::result::GrpcResult<Self> {
        AuthAsyncClient::new(host, port, tls).map(|c| {
            AuthClient {
                async_client: c,
            }
        })
    }
}

impl Auth for AuthClient {
    fn AuthEnable(&self, p: super::rpc::AuthEnableRequest) -> ::grpc::result::GrpcResult<super::rpc::AuthEnableResponse> {
        ::futures::Future::wait(self.async_client.AuthEnable(p))
    }

    fn AuthDisable(&self, p: super::rpc::AuthDisableRequest) -> ::grpc::result::GrpcResult<super::rpc::AuthDisableResponse> {
        ::futures::Future::wait(self.async_client.AuthDisable(p))
    }

    fn Authenticate(&self, p: super::rpc::AuthenticateRequest) -> ::grpc::result::GrpcResult<super::rpc::AuthenticateResponse> {
        ::futures::Future::wait(self.async_client.Authenticate(p))
    }

    fn UserAdd(&self, p: super::rpc::AuthUserAddRequest) -> ::grpc::result::GrpcResult<super::rpc::AuthUserAddResponse> {
        ::futures::Future::wait(self.async_client.UserAdd(p))
    }

    fn UserGet(&self, p: super::rpc::AuthUserGetRequest) -> ::grpc::result::GrpcResult<super::rpc::AuthUserGetResponse> {
        ::futures::Future::wait(self.async_client.UserGet(p))
    }

    fn UserList(&self, p: super::rpc::AuthUserListRequest) -> ::grpc::result::GrpcResult<super::rpc::AuthUserListResponse> {
        ::futures::Future::wait(self.async_client.UserList(p))
    }

    fn UserDelete(&self, p: super::rpc::AuthUserDeleteRequest) -> ::grpc::result::GrpcResult<super::rpc::AuthUserDeleteResponse> {
        ::futures::Future::wait(self.async_client.UserDelete(p))
    }

    fn UserChangePassword(&self, p: super::rpc::AuthUserChangePasswordRequest) -> ::grpc::result::GrpcResult<super::rpc::AuthUserChangePasswordResponse> {
        ::futures::Future::wait(self.async_client.UserChangePassword(p))
    }

    fn UserGrantRole(&self, p: super::rpc::AuthUserGrantRoleRequest) -> ::grpc::result::GrpcResult<super::rpc::AuthUserGrantRoleResponse> {
        ::futures::Future::wait(self.async_client.UserGrantRole(p))
    }

    fn UserRevokeRole(&self, p: super::rpc::AuthUserRevokeRoleRequest) -> ::grpc::result::GrpcResult<super::rpc::AuthUserRevokeRoleResponse> {
        ::futures::Future::wait(self.async_client.UserRevokeRole(p))
    }

    fn RoleAdd(&self, p: super::rpc::AuthRoleAddRequest) -> ::grpc::result::GrpcResult<super::rpc::AuthRoleAddResponse> {
        ::futures::Future::wait(self.async_client.RoleAdd(p))
    }

    fn RoleGet(&self, p: super::rpc::AuthRoleGetRequest) -> ::grpc::result::GrpcResult<super::rpc::AuthRoleGetResponse> {
        ::futures::Future::wait(self.async_client.RoleGet(p))
    }

    fn RoleList(&self, p: super::rpc::AuthRoleListRequest) -> ::grpc::result::GrpcResult<super::rpc::AuthRoleListResponse> {
        ::futures::Future::wait(self.async_client.RoleList(p))
    }

    fn RoleDelete(&self, p: super::rpc::AuthRoleDeleteRequest) -> ::grpc::result::GrpcResult<super::rpc::AuthRoleDeleteResponse> {
        ::futures::Future::wait(self.async_client.RoleDelete(p))
    }

    fn RoleGrantPermission(&self, p: super::rpc::AuthRoleGrantPermissionRequest) -> ::grpc::result::GrpcResult<super::rpc::AuthRoleGrantPermissionResponse> {
        ::futures::Future::wait(self.async_client.RoleGrantPermission(p))
    }

    fn RoleRevokePermission(&self, p: super::rpc::AuthRoleRevokePermissionRequest) -> ::grpc::result::GrpcResult<super::rpc::AuthRoleRevokePermissionResponse> {
        ::futures::Future::wait(self.async_client.RoleRevokePermission(p))
    }
}

// async client

pub struct AuthAsyncClient {
    grpc_client: ::grpc::client::GrpcClient,
    method_AuthEnable: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::rpc::AuthEnableRequest, super::rpc::AuthEnableResponse>>,
    method_AuthDisable: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::rpc::AuthDisableRequest, super::rpc::AuthDisableResponse>>,
    method_Authenticate: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::rpc::AuthenticateRequest, super::rpc::AuthenticateResponse>>,
    method_UserAdd: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::rpc::AuthUserAddRequest, super::rpc::AuthUserAddResponse>>,
    method_UserGet: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::rpc::AuthUserGetRequest, super::rpc::AuthUserGetResponse>>,
    method_UserList: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::rpc::AuthUserListRequest, super::rpc::AuthUserListResponse>>,
    method_UserDelete: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::rpc::AuthUserDeleteRequest, super::rpc::AuthUserDeleteResponse>>,
    method_UserChangePassword: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::rpc::AuthUserChangePasswordRequest, super::rpc::AuthUserChangePasswordResponse>>,
    method_UserGrantRole: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::rpc::AuthUserGrantRoleRequest, super::rpc::AuthUserGrantRoleResponse>>,
    method_UserRevokeRole: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::rpc::AuthUserRevokeRoleRequest, super::rpc::AuthUserRevokeRoleResponse>>,
    method_RoleAdd: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::rpc::AuthRoleAddRequest, super::rpc::AuthRoleAddResponse>>,
    method_RoleGet: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::rpc::AuthRoleGetRequest, super::rpc::AuthRoleGetResponse>>,
    method_RoleList: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::rpc::AuthRoleListRequest, super::rpc::AuthRoleListResponse>>,
    method_RoleDelete: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::rpc::AuthRoleDeleteRequest, super::rpc::AuthRoleDeleteResponse>>,
    method_RoleGrantPermission: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::rpc::AuthRoleGrantPermissionRequest, super::rpc::AuthRoleGrantPermissionResponse>>,
    method_RoleRevokePermission: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::rpc::AuthRoleRevokePermissionRequest, super::rpc::AuthRoleRevokePermissionResponse>>,
}

impl AuthAsyncClient {
    pub fn new(host: &str, port: u16, tls: bool) -> ::grpc::result::GrpcResult<Self> {
        ::grpc::client::GrpcClient::new(host, port, tls).map(|c| {
            AuthAsyncClient {
                grpc_client: c,
                method_AuthEnable: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/etcdserverpb.Auth/AuthEnable".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_AuthDisable: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/etcdserverpb.Auth/AuthDisable".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_Authenticate: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/etcdserverpb.Auth/Authenticate".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_UserAdd: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/etcdserverpb.Auth/UserAdd".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_UserGet: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/etcdserverpb.Auth/UserGet".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_UserList: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/etcdserverpb.Auth/UserList".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_UserDelete: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/etcdserverpb.Auth/UserDelete".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_UserChangePassword: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/etcdserverpb.Auth/UserChangePassword".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_UserGrantRole: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/etcdserverpb.Auth/UserGrantRole".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_UserRevokeRole: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/etcdserverpb.Auth/UserRevokeRole".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_RoleAdd: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/etcdserverpb.Auth/RoleAdd".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_RoleGet: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/etcdserverpb.Auth/RoleGet".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_RoleList: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/etcdserverpb.Auth/RoleList".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_RoleDelete: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/etcdserverpb.Auth/RoleDelete".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_RoleGrantPermission: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/etcdserverpb.Auth/RoleGrantPermission".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_RoleRevokePermission: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/etcdserverpb.Auth/RoleRevokePermission".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
            }
        })
    }
}

impl AuthAsync for AuthAsyncClient {
    fn AuthEnable(&self, p: super::rpc::AuthEnableRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::AuthEnableResponse> {
        self.grpc_client.call_unary(p, self.method_AuthEnable.clone())
    }

    fn AuthDisable(&self, p: super::rpc::AuthDisableRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::AuthDisableResponse> {
        self.grpc_client.call_unary(p, self.method_AuthDisable.clone())
    }

    fn Authenticate(&self, p: super::rpc::AuthenticateRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::AuthenticateResponse> {
        self.grpc_client.call_unary(p, self.method_Authenticate.clone())
    }

    fn UserAdd(&self, p: super::rpc::AuthUserAddRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::AuthUserAddResponse> {
        self.grpc_client.call_unary(p, self.method_UserAdd.clone())
    }

    fn UserGet(&self, p: super::rpc::AuthUserGetRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::AuthUserGetResponse> {
        self.grpc_client.call_unary(p, self.method_UserGet.clone())
    }

    fn UserList(&self, p: super::rpc::AuthUserListRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::AuthUserListResponse> {
        self.grpc_client.call_unary(p, self.method_UserList.clone())
    }

    fn UserDelete(&self, p: super::rpc::AuthUserDeleteRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::AuthUserDeleteResponse> {
        self.grpc_client.call_unary(p, self.method_UserDelete.clone())
    }

    fn UserChangePassword(&self, p: super::rpc::AuthUserChangePasswordRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::AuthUserChangePasswordResponse> {
        self.grpc_client.call_unary(p, self.method_UserChangePassword.clone())
    }

    fn UserGrantRole(&self, p: super::rpc::AuthUserGrantRoleRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::AuthUserGrantRoleResponse> {
        self.grpc_client.call_unary(p, self.method_UserGrantRole.clone())
    }

    fn UserRevokeRole(&self, p: super::rpc::AuthUserRevokeRoleRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::AuthUserRevokeRoleResponse> {
        self.grpc_client.call_unary(p, self.method_UserRevokeRole.clone())
    }

    fn RoleAdd(&self, p: super::rpc::AuthRoleAddRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::AuthRoleAddResponse> {
        self.grpc_client.call_unary(p, self.method_RoleAdd.clone())
    }

    fn RoleGet(&self, p: super::rpc::AuthRoleGetRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::AuthRoleGetResponse> {
        self.grpc_client.call_unary(p, self.method_RoleGet.clone())
    }

    fn RoleList(&self, p: super::rpc::AuthRoleListRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::AuthRoleListResponse> {
        self.grpc_client.call_unary(p, self.method_RoleList.clone())
    }

    fn RoleDelete(&self, p: super::rpc::AuthRoleDeleteRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::AuthRoleDeleteResponse> {
        self.grpc_client.call_unary(p, self.method_RoleDelete.clone())
    }

    fn RoleGrantPermission(&self, p: super::rpc::AuthRoleGrantPermissionRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::AuthRoleGrantPermissionResponse> {
        self.grpc_client.call_unary(p, self.method_RoleGrantPermission.clone())
    }

    fn RoleRevokePermission(&self, p: super::rpc::AuthRoleRevokePermissionRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::AuthRoleRevokePermissionResponse> {
        self.grpc_client.call_unary(p, self.method_RoleRevokePermission.clone())
    }
}

// sync server

pub struct AuthServer {
    async_server: AuthAsyncServer,
}

struct AuthServerHandlerToAsync {
    handler: ::std::sync::Arc<Auth + Send + Sync>,
    cpupool: ::futures_cpupool::CpuPool,
}

impl AuthAsync for AuthServerHandlerToAsync {
    fn AuthEnable(&self, p: super::rpc::AuthEnableRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::AuthEnableResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.AuthEnable(p)
        })
    }

    fn AuthDisable(&self, p: super::rpc::AuthDisableRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::AuthDisableResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.AuthDisable(p)
        })
    }

    fn Authenticate(&self, p: super::rpc::AuthenticateRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::AuthenticateResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.Authenticate(p)
        })
    }

    fn UserAdd(&self, p: super::rpc::AuthUserAddRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::AuthUserAddResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.UserAdd(p)
        })
    }

    fn UserGet(&self, p: super::rpc::AuthUserGetRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::AuthUserGetResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.UserGet(p)
        })
    }

    fn UserList(&self, p: super::rpc::AuthUserListRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::AuthUserListResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.UserList(p)
        })
    }

    fn UserDelete(&self, p: super::rpc::AuthUserDeleteRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::AuthUserDeleteResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.UserDelete(p)
        })
    }

    fn UserChangePassword(&self, p: super::rpc::AuthUserChangePasswordRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::AuthUserChangePasswordResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.UserChangePassword(p)
        })
    }

    fn UserGrantRole(&self, p: super::rpc::AuthUserGrantRoleRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::AuthUserGrantRoleResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.UserGrantRole(p)
        })
    }

    fn UserRevokeRole(&self, p: super::rpc::AuthUserRevokeRoleRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::AuthUserRevokeRoleResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.UserRevokeRole(p)
        })
    }

    fn RoleAdd(&self, p: super::rpc::AuthRoleAddRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::AuthRoleAddResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.RoleAdd(p)
        })
    }

    fn RoleGet(&self, p: super::rpc::AuthRoleGetRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::AuthRoleGetResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.RoleGet(p)
        })
    }

    fn RoleList(&self, p: super::rpc::AuthRoleListRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::AuthRoleListResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.RoleList(p)
        })
    }

    fn RoleDelete(&self, p: super::rpc::AuthRoleDeleteRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::AuthRoleDeleteResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.RoleDelete(p)
        })
    }

    fn RoleGrantPermission(&self, p: super::rpc::AuthRoleGrantPermissionRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::AuthRoleGrantPermissionResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.RoleGrantPermission(p)
        })
    }

    fn RoleRevokePermission(&self, p: super::rpc::AuthRoleRevokePermissionRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::rpc::AuthRoleRevokePermissionResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.RoleRevokePermission(p)
        })
    }
}

impl AuthServer {
    pub fn new<A : ::std::net::ToSocketAddrs, H : Auth + Send + Sync + 'static>(addr: A, h: H) -> Self {
        let h = AuthServerHandlerToAsync {
            cpupool: ::futures_cpupool::CpuPool::new_num_cpus(),
            handler: ::std::sync::Arc::new(h),
        };
        AuthServer {
            async_server: AuthAsyncServer::new(addr, h),
        }
    }
}

// async server

pub struct AuthAsyncServer {
    grpc_server: ::grpc::server::GrpcServer,
}

impl AuthAsyncServer {
    pub fn new<A : ::std::net::ToSocketAddrs, H : AuthAsync + 'static + Sync + Send + 'static>(addr: A, h: H) -> Self {
        let service_definition = AuthAsyncServer::new_service_def(h);
        AuthAsyncServer {
            grpc_server: ::grpc::server::GrpcServer::new(addr, service_definition),
        }
    }

    pub fn new_service_def<H : AuthAsync + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::server::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::server::ServerServiceDefinition::new(
            vec![
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/etcdserverpb.Auth/AuthEnable".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.AuthEnable(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/etcdserverpb.Auth/AuthDisable".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.AuthDisable(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/etcdserverpb.Auth/Authenticate".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.Authenticate(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/etcdserverpb.Auth/UserAdd".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.UserAdd(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/etcdserverpb.Auth/UserGet".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.UserGet(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/etcdserverpb.Auth/UserList".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.UserList(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/etcdserverpb.Auth/UserDelete".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.UserDelete(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/etcdserverpb.Auth/UserChangePassword".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.UserChangePassword(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/etcdserverpb.Auth/UserGrantRole".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.UserGrantRole(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/etcdserverpb.Auth/UserRevokeRole".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.UserRevokeRole(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/etcdserverpb.Auth/RoleAdd".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.RoleAdd(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/etcdserverpb.Auth/RoleGet".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.RoleGet(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/etcdserverpb.Auth/RoleList".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.RoleList(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/etcdserverpb.Auth/RoleDelete".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.RoleDelete(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/etcdserverpb.Auth/RoleGrantPermission".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.RoleGrantPermission(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/etcdserverpb.Auth/RoleRevokePermission".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.RoleRevokePermission(p))
                    },
                ),
            ],
        )
    }
}
