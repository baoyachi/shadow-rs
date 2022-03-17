macro_rules! concat_fn {
    ($fn_name:ident,$fn_desc:expr,$fn_body:expr) => {
        pub fn $fn_name() -> String {
            format!("{}\n{}", $fn_desc, $fn_body)
        }
    };
}

const VERSION_BRANCH_FN: &str = r##"#[allow(dead_code)]
pub fn version() -> &'static str {
    shadow_rs::const_format::concatcp!(r#"
pkg_version:{}
branch:{}
commit_hash:{}
build_time:{}
build_env:{},{}"#,PKG_VERSION, BRANCH, SHORT_COMMIT, BUILD_TIME, RUST_VERSION, RUST_CHANNEL
    )
}"##;

const VERSION_TAG_FN: &str = r##"#[allow(dead_code)]
pub fn version() -> &'static str {
    shadow_rs::const_format::concatcp!(r#"
pkg_version:{}
tag:{}
commit_hash:{}
build_time:{}
build_env:{},{}"#,PKG_VERSION, TAG, SHORT_COMMIT, BUILD_TIME, RUST_VERSION, RUST_CHANNEL
    )
}"##;

const CLAP_VERSION_BRANCH_FN: &str = r##"#[allow(dead_code)]
#[deprecated = "Replaced with `clap_long_version`"]
pub fn clap_version() -> &'static str {
    shadow_rs::const_format::concatcp!(r#"{}
branch:{}
commit_hash:{}
build_time:{}
build_env:{},{}"#,PKG_VERSION, BRANCH, SHORT_COMMIT, BUILD_TIME, RUST_VERSION, RUST_CHANNEL
    )
}"##;

const CLAP_VERSION_TAG_FN: &str = r##"#[allow(dead_code)]
#[deprecated = "Replaced with `clap_long_version`"]
pub fn clap_version() -> &'static str {
    shadow_rs::const_format::concatcp!(r#"{}
tag:{}
commit_hash:{}
build_time:{}
build_env:{},{}"#,PKG_VERSION, TAG, SHORT_COMMIT, BUILD_TIME, RUST_VERSION, RUST_CHANNEL
    )
}"##;

const CLAP_LONG_VERSION_BRANCH_FN: &str = r##"#[allow(dead_code)]
pub fn clap_long_version() -> &'static str {
    shadow_rs::const_format::concatcp!(r#"{}
branch:{}
commit_hash:{}
build_time:{}
build_env:{},{}"#,PKG_VERSION, BRANCH, SHORT_COMMIT, BUILD_TIME, RUST_VERSION, RUST_CHANNEL
    )
}"##;

const CLAP_LONG_VERSION_TAG_FN: &str = r##"#[allow(dead_code)]
pub fn clap_long_version() -> &'static str {
    shadow_rs::const_format::concatcp!(r#"{}
tag:{}
commit_hash:{}
build_time:{}
build_env:{},{}"#,PKG_VERSION, TAG, SHORT_COMMIT, BUILD_TIME, RUST_VERSION, RUST_CHANNEL
    )
}"##;

const VERSION_FN_DESC: &str =
    r#"/// The common version function. It's so easy to use this function."#;

const CLAP_VERSION_FN_DESC: &str =
    r#"/// The common version function. It's so easy to use this function with clap_version()."#;

const CLAP_LONG_VERSION_FN_DESC: &str =
    r#"/// The common version function. It's so easy to use this function with clap_version()."#;

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

concat_fn!(
    clap_long_version_branch_fn,
    CLAP_LONG_VERSION_FN_DESC,
    CLAP_LONG_VERSION_BRANCH_FN
);
concat_fn!(
    clap_long_version_tag_fn,
    CLAP_LONG_VERSION_FN_DESC,
    CLAP_LONG_VERSION_TAG_FN
);

pub(crate) const BUILD_FN_VERSION: &str = "version";
pub(crate) const BUILD_FN_CLAP_LONG_VERSION: &str = "clap_long_version";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_fn() {
        assert!(version_branch_fn().contains(VERSION_FN_DESC));
        assert!(version_tag_fn().contains(VERSION_TAG_FN));
        assert!(clap_version_branch_fn().contains(CLAP_VERSION_BRANCH_FN));
        assert!(clap_version_tag_fn().contains(CLAP_VERSION_FN_DESC));
        assert!(clap_long_version_branch_fn().contains(CLAP_LONG_VERSION_BRANCH_FN));
        assert!(clap_long_version_tag_fn().contains(CLAP_LONG_VERSION_FN_DESC));
    }
}
