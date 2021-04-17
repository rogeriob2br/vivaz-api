fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/entrybuffer.proto").unwrap();
    tonic_build::compile_protos("proto/cachebuffer.proto").unwrap();
    tonic_build::compile_protos("proto/pubeventbuffer.proto").unwrap();

    Ok(())
}