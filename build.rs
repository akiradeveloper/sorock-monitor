fn main() {
    tonic_build::configure()
        .build_server(false)
        .compile_protos(&["sorock_monitor.proto"], &["proto"])
        .unwrap();
}
