fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/user.proto")?;
    tonic_build::compile_protos("proto/trader.proto")?;
    Ok(())
}