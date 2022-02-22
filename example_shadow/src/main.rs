use clap::App;

use shadow_rs::shadow;

shadow!(build);

fn main() {
    App::new("example_shadow")
        .long_version(build::clap_long_version().as_str())
        .get_matches(); //USAGE: ./example_shadow -V

    // shadow-rs built in function
    println!("is_debug:{}", shadow_rs::is_debug());
    println!("branch:{}", shadow_rs::branch());
    println!("tag:{}", shadow_rs::tag());
    println!("git_clean:{}", shadow_rs::git_clean());
    println!("git_status_file:{}", shadow_rs::git_status_file());

    // print_build()

    build::print_build_in();
}

#[allow(dead_code)]
pub fn print_build() {
    println!("clap_long_version:{}", build::clap_long_version());
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
    println!("commit_date_2822:{}", build::COMMIT_DATE_2822);
    println!("commit_date_3339:{}", build::COMMIT_DATE_3339);
    println!("commit_author:{}", build::COMMIT_AUTHOR);
    println!("commit_email:{}", build::COMMIT_EMAIL);

    println!("build_os:{}", build::BUILD_OS);
    println!("rust_version:{}", build::RUST_VERSION);
    println!("rust_channel:{}", build::RUST_CHANNEL);
    println!("cargo_version:{}", build::CARGO_VERSION);
    println!("cargo_tree:{}", build::CARGO_TREE);

    println!("project_name:{}", build::PROJECT_NAME);
    println!("build_time:{}", build::BUILD_TIME);
    println!("build_time_2822:{}", build::BUILD_TIME_2822);
    println!("build_time_3339:{}", build::BUILD_TIME_3339);
    println!("build_rust_channel:{}", build::BUILD_RUST_CHANNEL);
}
