mod shadow;
mod channel;
mod git;


pub const SHADOW_PREFIX: &str = "SHADOW_RS_";

#[cfg(test)]
mod tests {
    #[test]
    fn test_hello() {
        println!("{}", env!("CARGO_PKG_NAME"));
    }
}