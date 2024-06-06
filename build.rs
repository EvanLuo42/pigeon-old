use std::io::Result;

fn main() -> Result<()> {
    prost_build::compile_protos(
        &["protos/common.proto"],
        &["protos/"]
    )?;
    Ok(())
}