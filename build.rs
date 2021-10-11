fn main() {
    // compile protocol buffer using protoc
    protoc_rust_grpc::Codegen::new()
        .out_dir("src/gen")
        .inputs(vec!["./proto/language.proto", "./proto/plugin.proto"])
        .include("./proto")
        .rust_protobuf(true)
        .run()
        .expect("error compiling protocol buffer");
}
