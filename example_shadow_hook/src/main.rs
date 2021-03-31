use shadow_rs::shadow;

shadow!(build);

fn main() {
    println!("const:{}", build::HOOK_CONST); //expect:'hello hook const'
    println!("fn:{}", build::hook_fn()); //expect:'hello hook bar fn'
}
