use anyhow::Result;

fn main() -> Result<()>{
    tonic_build::compile_protos("proto/dopple.proto")?;
    Ok(())
}
