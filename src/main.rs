use std::fs;

mod git;
mod shadow;
mod channel;

fn main() {
    // git::Git::print_git();
    println!("{:?}",shadow::Environment::new());




}