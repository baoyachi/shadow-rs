mod shadow;
mod channel;

#[cfg(test)]
mod tests {
    #[test]
    fn test_hello() {
        println!("{}",env!("CARGO_PKG_NAME"));
    }
}