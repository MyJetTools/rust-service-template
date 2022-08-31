fn main() {
    let example_proto_file = "./proto/example.proto"; 

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir("./generated_proto/src")
        .compile(&[example_proto_file], &["."])
        .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));

    println!("cargo:rerun-if-changed={}", example_proto_file);
}