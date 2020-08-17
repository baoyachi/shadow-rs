extern crate shadow_rs;

use shadow_rs::Shadow;
use std::fs;

fn main() -> shadow_rs::err::SdResult<()> {
    let src_path = std::env::var("CARGO_MANIFEST_DIR")?;

    Shadow::build(src_path, "./".to_string())?;

    for (k, v) in std::env::vars_os() {
        println!("{:?},{:?}", k, v);
    }
    println!("{}", fs::read_to_string("./shadow.rs")?);

    Ok(())
}
