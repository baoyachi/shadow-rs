fn main() {
    for (k, v) in std_env::vars() {
        println!("K:{},V:{}", k, v);
    }
}