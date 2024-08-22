macro_rules! gen_const {
    ($fn_name:ident, $fn_body:expr) => {
        pub fn $fn_name() -> String {
            let (doc, content) = $fn_body;
            format!(
                "{}\n{}\n{}\n{}\n",
                doc,
                "#[allow(dead_code,missing_docs)]",
                $crate::CARGO_CLIPPY_ALLOW_ALL,
                content
            )
        }
    };
}

const VERSION_BRANCH_CONST: (&str, &str) = (
    r#"/// A long version string describing the project.
/// The version string contains the package version, branch, commit hash, build time, and build environment on separate lines.
/// This constant is suitable for printing to the user."#,
    r##"pub const VERSION:&str = shadow_rs::formatcp!(r#"
pkg_version:{}
branch:{}
commit_hash:{}
build_time:{}
build_env:{},{}"#,PKG_VERSION, BRANCH, SHORT_COMMIT, BUILD_TIME, RUST_VERSION, RUST_CHANNEL
);"##,
);

const VERSION_TAG_CONST: (&str, &str) = (
    r#"/// A long version string describing the project.
/// The version string contains the package version, current Git tag, commit hash, build time, and build environment on separate lines.
/// This constant is suitable for printing to the user."#,
    r##"pub const VERSION:&str = shadow_rs::formatcp!(r#"
pkg_version:{}
tag:{}
commit_hash:{}
build_time:{}
build_env:{},{}"#,PKG_VERSION, TAG, SHORT_COMMIT, BUILD_TIME, RUST_VERSION, RUST_CHANNEL
);"##,
);

const CLAP_VERSION_BRANCH_CONST: (&str, &str) = (
    r#"#[deprecated = "Replaced with `CLAP_LONG_VERSION`"]"#,
    r##"pub const CLAP_VERSION:&str = shadow_rs::formatcp!(r#"{}
branch:{}
commit_hash:{}
build_time:{}
build_env:{},{}"#,PKG_VERSION, BRANCH, SHORT_COMMIT, BUILD_TIME, RUST_VERSION, RUST_CHANNEL
);"##,
);

const CLAP_VERSION_TAG_CONST: (&str, &str) = (
    r#"#[deprecated = "Replaced with `CLAP_LONG_VERSION`"]"#,
    r##"pub const CLAP_VERSION:&str = shadow_rs::formatcp!(r#"{}
tag:{}
commit_hash:{}
build_time:{}
build_env:{},{}"#,PKG_VERSION, TAG, SHORT_COMMIT, BUILD_TIME, RUST_VERSION, RUST_CHANNEL
);"##,
);

const CLAP_LONG_VERSION_BRANCH_CONST: (&str, &str) = (
    r#"/// A long version string describing the project.
/// The version string contains the package version, branch, commit hash, build time, and build environment on separate lines.
/// This constant is intended to be used by clap or other CLI tools as a long version string."#,
    r##"pub const CLAP_LONG_VERSION:&str = shadow_rs::formatcp!(r#"{}
branch:{}
commit_hash:{}
build_time:{}
build_env:{},{}"#,PKG_VERSION, BRANCH, SHORT_COMMIT, BUILD_TIME, RUST_VERSION, RUST_CHANNEL
);"##,
);

const CLAP_LONG_VERSION_TAG_CONST: (&str, &str) = (
    r#"/// A long version string describing the project.
/// The version string contains the package version, current Git tag, commit hash, build time, and build environment on separate lines.
/// This constant is intended to be used by clap or other CLI tools as a long version string."#,
    r##"pub const CLAP_LONG_VERSION:&str = shadow_rs::formatcp!(r#"{}
tag:{}
commit_hash:{}
build_time:{}
build_env:{},{}"#,PKG_VERSION, TAG, SHORT_COMMIT, BUILD_TIME, RUST_VERSION, RUST_CHANNEL
);"##,
);

gen_const!(version_branch_const, VERSION_BRANCH_CONST);
gen_const!(version_tag_const, VERSION_TAG_CONST);
gen_const!(clap_version_branch_const, CLAP_VERSION_BRANCH_CONST);
gen_const!(clap_version_tag_const, CLAP_VERSION_TAG_CONST);
gen_const!(
    clap_long_version_branch_const,
    CLAP_LONG_VERSION_BRANCH_CONST
);
gen_const!(clap_long_version_tag_const, CLAP_LONG_VERSION_TAG_CONST);

pub(crate) const BUILD_CONST_VERSION: &str = "VERSION";
pub(crate) const BUILD_CONST_CLAP_LONG_VERSION: &str = "CLAP_LONG_VERSION";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_fn() {
        assert!(version_tag_const().contains(VERSION_TAG_CONST.0));
        assert!(clap_version_branch_const().contains(CLAP_VERSION_BRANCH_CONST.1));
        assert!(clap_long_version_branch_const().contains(CLAP_LONG_VERSION_BRANCH_CONST.1));
    }
}
