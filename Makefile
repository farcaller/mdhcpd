PROTOC ?= protoc

all: src/proto/auth.rs src/proto/kv.rs src/proto/rpc.rs src/proto/rpc_grpc.rs

src/proto/auth.rs: thirdparty/etcd/auth/authpb/auth.proto
	$(PROTOC) --rust_out=src/proto -Ithirdparty/ -Ithirdparty/gogo-protobuf -Ithirdparty/googleapis $<
src/proto/kv.rs: thirdparty/etcd/mvcc/mvccpb/kv.proto
	$(PROTOC) --rust_out=src/proto -Ithirdparty/ -Ithirdparty/gogo-protobuf -Ithirdparty/googleapis $<
src/proto/rpc.rs: thirdparty/etcd/etcdserver/etcdserverpb/rpc.proto
	$(PROTOC) --rust_out=src/proto -Ithirdparty/ -Ithirdparty/gogo-protobuf -Ithirdparty/googleapis $<
src/proto/rpc_grpc.rs: thirdparty/etcd/etcdserver/etcdserverpb/rpc.proto
	$(PROTOC) --rust-grpc_out=src/proto -Ithirdparty/ -Ithirdparty/gogo-protobuf -Ithirdparty/googleapis $<
