use shadow_rs::{SdResult, ShadowBuilder};
use std::fs::File;
use std::io::Write;

fn main() {
    ShadowBuilder::builder().hook(hook).build().unwrap();
}

fn hook(file: &File) -> SdResult<()> {
    append_write_const(file)?;
    append_write_fn(file)?;
    Ok(())
}

fn append_write_const(mut file: &File) -> SdResult<()> {
    let hook_const: &str = r#"#[allow(clippy::all, clippy::pedantic, clippy::restriction, clippy::nursery)]
pub const HOOK_CONST: &str = "hello hook const";"#;
    writeln!(file, "{hook_const}")?;
    Ok(())
}

fn append_write_fn(mut file: &File) -> SdResult<()> {
    let hook_fn: &str = r#"
pub fn hook_fn() -> &'static str{
    HOOK_CONST
}"#;
    writeln!(file, "{hook_fn}")?;
    Ok(())
}
