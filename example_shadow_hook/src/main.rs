#![allow(clippy::needless_raw_strings)]
#![allow(clippy::needless_raw_string_hashes)]

use shadow_rs::shadow;

shadow!(build);

fn main() {
    // how to use new_hook function: https://github.com/baoyachi/shadow-rs/blob/master/example_shadow_hook/build.rs
    println!("const:{}", build::HOOK_CONST); //expect:'hello hook const'
    println!("fn:{}", build::hook_fn()); //expect:'hello hook bar fn'
}
