mod shadow;
mod channel;
mod git;

#[cfg(test)]
mod tests {
    #[test]
    fn test_hello() {
        println!("{}",env!("CARGO_PKG_NAME"));
    }
}