mod shadow;
mod channel;
mod git;


pub const SHADOW_RS_ENV_PREFIX: &str = "SHADOW_RUST_ENV";

#[cfg(test)]
mod tests {
    #[test]
    fn test_hello() {
        println!("{}", env!("CARGO_PKG_NAME"));
    }
}