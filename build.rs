fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/planes.proto")?;
    println!("cargo:rerun-if-changed=migrations");
    Ok(())
}