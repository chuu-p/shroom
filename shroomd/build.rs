fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure().compile_protos(&["proto/ShroomService.proto"], &["proto"])?;
    Ok(())
}
