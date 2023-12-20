use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // compile protos
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional")
        .file_descriptor_set_path(out_dir.join("planes_descriptor.bin"))
        .compile(&["proto/planes.proto"], &["proto"])?;

    // rerun if migrations change
    println!("cargo:rerun-if-changed=migrations");

    Ok(())
}
