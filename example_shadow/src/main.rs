use clap::App;

#[macro_use]
extern crate shadow_rs;

shadow!(build);

fn main() {
    App::new("example_shadow")
        .version(build::version().as_str())
        .get_matches(); //USAGE: ./example_shadow -V

    // shadow-rs built in method
    println!("is_debug:{}", shadow_rs::is_debug());
    println!("branch:{}", shadow_rs::branch());

    println!("version:{}", build::version());
    println!("pkg_version:{}", build::PKG_VERSION);
    println!("pkg_version_major:{}", build::PKG_VERSION_MAJOR);
    println!("pkg_version_minor:{}", build::PKG_VERSION_MINOR);
    println!("pkg_version_patch:{}", build::PKG_VERSION_PATCH);
    println!("pkg_version_pre:{}", build::PKG_VERSION_PRE);

    println!("tag:{}", build::TAG);
    println!("branch:{}", build::BRANCH);
    println!("commit_id:{}", build::COMMIT_HASH);
    println!("short_commit:{}", build::SHORT_COMMIT);
    println!("commit_date:{}", build::COMMIT_DATE);
    println!("commit_author:{}", build::COMMIT_AUTHOR);
    println!("commit_email:{}", build::COMMIT_EMAIL);

    println!("build_os:{}", build::BUILD_OS);
    println!("rust_version:{}", build::RUST_VERSION);
    println!("rust_channel:{}", build::RUST_CHANNEL);
    println!("cargo_version:{}", build::CARGO_VERSION);
    println!("cargo_tree:{}", build::CARGO_TREE);

    println!("project_name:{}", build::PROJECT_NAME);
    println!("build_time:{}", build::BUILD_TIME);
    println!("build_rust_channel:{}", build::BUILD_RUST_CHANNEL);
}
