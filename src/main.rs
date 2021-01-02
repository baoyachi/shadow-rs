use std::path::Path;

fn main() {
    let repo = git2::Repository::discover(Path::new("./")).unwrap();
    let reference = repo.head().unwrap();
    let option = reference.shorthand();
    println!("shorthand:{:?}",option);

    for (k, v) in std::env::vars() {
        println!("K:{},V:{}", k, v);
    }
}