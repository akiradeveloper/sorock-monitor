fn main() {
    tonic_build::configure()
        .compile_protos(&["sorock_monitor.proto"], &["proto"])
        .unwrap();
}
