fn main() -> std::io::Result<()> {
    prost_build::compile_protos(
        &["protos/test.proto", "protos/main.proto"],
        &["protos/"]
    )?;
    Ok(())
}