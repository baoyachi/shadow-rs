pub mod shadow{
    include!(concat!(env!("OUT_DIR"), "/shadow.rs"));
}

fn main() {
    println!("{}",shadow::BRANCH);
    println!("{}",shadow::COMMIT_HASH);
    println!("{}",shadow::COMMIT_DATE);
    println!("{}",shadow::COMMIT_AUTHOR);
    println!("{}",shadow::COMMIT_EMAIL);

    println!("{}",shadow::BUILD_OS);
    println!("{}",shadow::RUST_VERSION);
    println!("{}",shadow::RUST_CHANNEL);
    println!("{}",shadow::CARGO_VERSION);
    println!("{:?}",shadow::CARGO_LOCK);

    println!("{:?}",shadow::PROJECT_NAME);
    println!("{:?}",shadow::BUILD_TIME);
    println!("{:?}",shadow::BUILD_RUST_CHANNEL);

}