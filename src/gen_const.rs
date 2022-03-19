macro_rules! gen_const {
    ($fn_name:ident,$fn_desc:expr,$fn_body:expr) => {
        pub fn $fn_name() -> String {
            format!("{}\n{}", $fn_desc, $fn_body)
        }
    };
}

const VERSION_BRANCH_CONST: &str = r##"#[allow(dead_code)]
pub const VERSION:&str = shadow_rs::formatcp!(r#"
pkg_version:{}
branch:{}
commit_hash:{}
build_time:{}
build_env:{},{}"#,PKG_VERSION, BRANCH, SHORT_COMMIT, BUILD_TIME, RUST_VERSION, RUST_CHANNEL
);"##;

const VERSION_TAG_CONST: &str = r##"#[allow(dead_code)]
pub const VERSION:&str = shadow_rs::formatcp!(r#"
pkg_version:{}
tag:{}
commit_hash:{}
build_time:{}
build_env:{},{}"#,PKG_VERSION, TAG, SHORT_COMMIT, BUILD_TIME, RUST_VERSION, RUST_CHANNEL
);"##;

const CLAP_VERSION_BRANCH_CONST: &str = r##"#[allow(dead_code)]
#[deprecated = "Replaced with `CLAP_LONG_VERSION`"]
pub const CLAP_VERSION:&str = shadow_rs::formatcp!(r#"{}
branch:{}
commit_hash:{}
build_time:{}
build_env:{},{}"#,PKG_VERSION, BRANCH, SHORT_COMMIT, BUILD_TIME, RUST_VERSION, RUST_CHANNEL
);"##;

const CLAP_VERSION_TAG_CONST: &str = r##"#[allow(dead_code)]
#[deprecated = "Replaced with `CLAP_LONG_VERSION`"]
pub const CLAP_VERSION:&str = shadow_rs::formatcp!(r#"{}
tag:{}
commit_hash:{}
build_time:{}
build_env:{},{}"#,PKG_VERSION, TAG, SHORT_COMMIT, BUILD_TIME, RUST_VERSION, RUST_CHANNEL
);"##;

const CLAP_LONG_VERSION_BRANCH_CONST: &str = r##"#[allow(dead_code)]
pub const CLAP_LONG_VERSION:&str = shadow_rs::formatcp!(r#"{}
branch:{}
commit_hash:{}
build_time:{}
build_env:{},{}"#,PKG_VERSION, BRANCH, SHORT_COMMIT, BUILD_TIME, RUST_VERSION, RUST_CHANNEL
);"##;

const CLAP_LONG_VERSION_TAG_CONST: &str = r##"#[allow(dead_code)]
pub const CLAP_LONG_VERSION:&str = shadow_rs::formatcp!(r#"{}
tag:{}
commit_hash:{}
build_time:{}
build_env:{},{}"#,PKG_VERSION, TAG, SHORT_COMMIT, BUILD_TIME, RUST_VERSION, RUST_CHANNEL
);"##;

const VERSION_CONST_DESC: &str = r#"/// The common version const. It's so easy to use this const."#;

const CLAP_VERSION_CONST_DESC: &str =
    r#"/// The common version const. It's so easy to use this const with CLAP_VERSION."#;

const CLAP_LONG_VERSION_CONST_DESC: &str =
    r#"/// The common version const. It's so easy to use this const with CLAP_VERSION."#;

gen_const!(
    version_branch_const,
    VERSION_CONST_DESC,
    VERSION_BRANCH_CONST
);

gen_const!(version_tag_const, VERSION_CONST_DESC, VERSION_TAG_CONST);

gen_const!(
    clap_version_branch_const,
    CLAP_VERSION_CONST_DESC,
    CLAP_VERSION_BRANCH_CONST
);
gen_const!(
    clap_version_tag_const,
    CLAP_VERSION_CONST_DESC,
    CLAP_VERSION_TAG_CONST
);

gen_const!(
    clap_long_version_branch_const,
    CLAP_LONG_VERSION_CONST_DESC,
    CLAP_LONG_VERSION_BRANCH_CONST
);
gen_const!(
    clap_long_version_tag_const,
    CLAP_LONG_VERSION_CONST_DESC,
    CLAP_LONG_VERSION_TAG_CONST
);

pub(crate) const BUILD_CONST_VERSION: &str = "VERSION";
pub(crate) const BUILD_CONST_CLAP_LONG_VERSION: &str = "CLAP_LONG_VERSION";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_fn() {
        assert!(version_branch_const().contains(VERSION_CONST_DESC));
        assert!(version_tag_const().contains(VERSION_TAG_CONST));
        assert!(clap_version_branch_const().contains(CLAP_VERSION_BRANCH_CONST));
        assert!(clap_version_tag_const().contains(CLAP_VERSION_CONST_DESC));
        assert!(clap_long_version_branch_const().contains(CLAP_LONG_VERSION_BRANCH_CONST));
        assert!(clap_long_version_tag_const().contains(CLAP_LONG_VERSION_CONST_DESC));
    }
}
