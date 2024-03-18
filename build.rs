fn main() -> Result<(), Box<dyn std::error::Error>> {
    // rerun if migrations change
    println!("cargo:rerun-if-changed=migrations");

    Ok(())
}
