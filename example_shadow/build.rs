use shadow_rs::SdResult;
use std::fs::File;
use std::io::Write;

fn main() -> shadow_rs::SdResult<()> {
    shadow_rs::new_hook(append_write_const)
}

fn append_write_const(mut file: &File) -> SdResult<()> {
    const CONST_FOO: &str = r#"pub const TEST_CONST: &str = "CONST_FOO";"#;

    writeln!(file, "{}", CONST_FOO)?;
    Ok(())
}
