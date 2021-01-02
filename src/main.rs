fn main() {
    for (k, v) in std::env::vars() {
        println!("K:{},V:{}", k, v);
    }
}