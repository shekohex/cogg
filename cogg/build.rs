#![warn(rust_2018_idioms)]
fn main() {
    println!("Start Compiling Protos");
    let proto_root = "src/protos";
    let proto_main = "src/protos/cogg/main.proto";
    println!("cargo:rerun-if-changed={}", proto_main);
    protoc_grpcio::compile_grpc_protos(&["cogg/main.proto"], &[proto_root], &proto_root)
        .expect("Failed to compile gRPC definitions!");
}
