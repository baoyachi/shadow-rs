fn main() {
    for (k, v) in std::env::vars_os() {
        println!("{:?},{:?}", k, v);
    }

}