use shadow_rs::err::SdResult;
use std::error::Error;

fn main() -> shadow_rs::err::SdResult<()> {
    shadow_rs::Shadow::new(&std::env::var("OUT_DIR").unwrap())?;
    Ok(())
}
