#![warn(rust_2018_idioms)]
fn main() {
    println!("Start Compiling Protos");
    let proto_root = "src/protos";
    let proto_users = "src/protos/cogg/files.proto";
    let proto_files = "src/protos/cogg/users.proto";
    let proto_processes = "src/protos/cogg/processes.proto";
    println!("cargo:rerun-if-changed={}", proto_files);
    println!("cargo:rerun-if-changed={}", proto_users);
    println!("cargo:rerun-if-changed={}", proto_processes);
    protoc_grpcio::compile_grpc_protos(
        &["cogg/files.proto", "cogg/users.proto", "cogg/processes.proto"],
        &[proto_root],
        &proto_root,
    ).expect("Failed to compile gRPC definitions!");
}
