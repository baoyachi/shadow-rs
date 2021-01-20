macro_rules! concat_fn {
    ($fn_name:ident,$fn_desc:expr,$fn_body:expr) => {
        pub fn $fn_name() -> String {
            format!("{}{}", $fn_desc, $fn_body)
        }
    };
}

const VERSION_BRANCH_FN: &str = r##"#[allow(dead_code)]
pub fn version() -> String {
    format!(r#"
pkg_version:{}
branch:{}
commit_hash:{}
build_time:{}
build_env:{},{}"#,PKG_VERSION, BRANCH, SHORT_COMMIT, BUILD_TIME, RUST_VERSION, RUST_CHANNEL
    )
}"##;

const VERSION_TAG_FN: &str = r##"#[allow(dead_code)]
pub fn version() -> String {
    format!(r#"
pkg_version:{}
tag:{}
commit_hash:{}
build_time:{}
build_env:{},{}"#,PKG_VERSION, TAG, SHORT_COMMIT, BUILD_TIME, RUST_VERSION, RUST_CHANNEL
    )
}"##;

const CLAP_VERSION_BRANCH_FN: &str = r##"#[allow(dead_code)]
pub fn clap_version() -> String {
    format!(r#"{}
branch:{}
commit_hash:{}
build_time:{}
build_env:{},{}"#,PKG_VERSION, BRANCH, SHORT_COMMIT, BUILD_TIME, RUST_VERSION, RUST_CHANNEL
    )
}"##;

const CLAP_VERSION_TAG_FN: &str = r##"#[allow(dead_code)]
pub fn clap_version() -> String {
    format!(r#"{}
tag:{}
commit_hash:{}
build_time:{}
build_env:{},{}"#,PKG_VERSION, TAG, SHORT_COMMIT, BUILD_TIME, RUST_VERSION, RUST_CHANNEL
    )
}"##;

const VERSION_FN_DESC: &str = r#"
/// The common version function. It's so easy to use this function.
///
/// # Examples
///
/// ```ignore
///   fn main(){
///       println!("{}",shadow::version()); //print version() function
///   }
/// ```
"#;

const CLAP_VERSION_FN_DESC: &str = r#"
/// The common version function. It's so easy to use this function with clap verion().
///
/// # Examples
///
///```ignore
///   App::new("example_shadow")
///        .version(shadow::clap_version().as_str())
///        .get_matches()
///```
"#;

concat_fn!(version_branch_fn, VERSION_FN_DESC, VERSION_BRANCH_FN);

concat_fn!(version_tag_fn, VERSION_FN_DESC, VERSION_TAG_FN);

concat_fn!(
    clap_version_branch_fn,
    CLAP_VERSION_FN_DESC,
    CLAP_VERSION_BRANCH_FN
);
concat_fn!(
    clap_version_tag_fn,
    CLAP_VERSION_FN_DESC,
    CLAP_VERSION_TAG_FN
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_fn() {
        println!("{}", version_branch_fn());
        println!("{}", version_tag_fn());
        println!("{}", clap_version_branch_fn());
        println!("{}", clap_version_tag_fn());
    }
}
