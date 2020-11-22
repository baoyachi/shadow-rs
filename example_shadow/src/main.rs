use clap::App;

pub mod shadow {
    include!(concat!(env!("OUT_DIR"), "/shadow.rs"));
}

pub fn version() -> String {
    format!(
        r#"
branch:{}
commit-hash:{}
build_time:{}
build_env:{},{}
"#,
        shadow::BRANCH,
        shadow::SHORT_COMMIT,
        shadow::BUILD_TIME,
        shadow::RUST_VERSION,
        shadow::RUST_CHANNEL,
    )
}

fn main() {
    App::new("example_shadow")
        .version(version().as_str())
        .get_matches(); //USAGE: ./example_shadow -V

    println!("branch:{}", shadow::BRANCH);
    println!("commit_id:{}", shadow::COMMIT_HASH);
    println!("short_commit:{}", shadow::SHORT_COMMIT);
    println!("commit_date:{}", shadow::COMMIT_DATE);
    println!("commit_author:{}", shadow::COMMIT_AUTHOR);
    println!("commit_email:{}", shadow::COMMIT_EMAIL);

    println!("build_os:{}", shadow::BUILD_OS);
    println!("rust_version:{}", shadow::RUST_VERSION);
    println!("rust_channel:{}", shadow::RUST_CHANNEL);
    println!("cargo_version:{}", shadow::CARGO_VERSION);
    println!("pkg_version:{}", shadow::PKG_VERSION);
    println!("cargo_lock:{}", shadow::CARGO_LOCK);

    println!("project_name:{}", shadow::PROJECT_NAME);
    println!("build_time:{}", shadow::BUILD_TIME);
    println!("build_rust_channel:{}", shadow::BUILD_RUST_CHANNEL);
}
