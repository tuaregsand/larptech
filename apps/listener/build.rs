fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto = "../../libs/proto/mint.proto";
    prost_build::compile_protos(&[proto], &["../../libs/proto"])?;
    println!("cargo:rerun-if-changed={}", proto);
    Ok(())
}
