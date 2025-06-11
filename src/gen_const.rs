use crate::{Shadow, CARGO_CLIPPY_ALLOW_ALL, CARGO_METADATA};

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
gen_const!(
    clap_long_version_branch_const,
    CLAP_LONG_VERSION_BRANCH_CONST
);
gen_const!(clap_long_version_tag_const, CLAP_LONG_VERSION_TAG_CONST);

pub(crate) const BUILD_CONST_VERSION: &str = "VERSION";
pub(crate) const BUILD_CONST_CLAP_LONG_VERSION: &str = "CLAP_LONG_VERSION";

#[allow(dead_code)]
pub(crate) fn cargo_metadata_fn(shadow: &Shadow) -> String {
    if !shadow.map.contains_key(CARGO_METADATA) {
        return "".to_string();
    }
    format!(
        r#"
use shadow_rs::cargo_metadata::Metadata;
use shadow_rs::serde_json;

/// Attempts to parse the Cargo package metadata from the generated constant CARGO_METADATA.
///
/// Returns a `Metadata` struct containing information about the Cargo workspace,
/// such as details about the packages and their dependencies.
///
/// # Return Values
/// - `Ok(Metadata)`: Contains the parsed metadata if successful.
/// - `Err(String)`: Returns an error message if converting the environment variable to a UTF-8 string or parsing JSON fails.
#[allow(dead_code)]
{CARGO_CLIPPY_ALLOW_ALL}
pub fn cargo_metadata() -> Result<Metadata, String> {{
    let metadata_json = std::str::from_utf8(CARGO_METADATA.as_ref()).map_err(|err| format!("generate 'CARGO_METADATA' value from UTF8 error:{{}}",err))?;
    let meta: Metadata = serde_json::from_str(metadata_json).map_err(|err| err.to_string())?;
    Ok(meta)
}}"#
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_fn() {
        assert!(version_tag_const().contains(VERSION_TAG_CONST.0));
        assert!(clap_long_version_branch_const().contains(CLAP_LONG_VERSION_BRANCH_CONST.1));
    }

    #[cfg(feature = "metadata")]
    #[test]
    fn test_metadata() {
        let metadata_json = r#"{"metadata":null,"packages":[{"authors":["baoyachi"],"categories":["development-tools","development-tools::build-utils"],"default_run":null,"dependencies":[{"features":[],"kind":"dev","name":"xshell","optional":false,"registry":null,"rename":null,"req":"^0.2.7","source":"registry+https://github.com/rust-lang/crates.io-index","target":null,"uses_default_features":true}],"description":"get build model is debug","documentation":null,"edition":"2018","features":{"default":["std"],"std":[]},"homepage":null,"id":"registry+https://github.com/rust-lang/crates.io-index#is_debug@1.1.0","keywords":["build","build-model","model","debug","release"],"license":"MIT AND Apache-2.0","license_file":null,"links":null,"manifest_path":".cargo/registry/src/index.crates.io-1949cf8c6b5b557f/is_debug-1.1.0/Cargo.toml","metadata":null,"name":"is_debug","publish":null,"readme":"README.md","repository":"https://github.com/baoyachi/rust_is_debug","rust_version":null,"source":"registry+https://github.com/rust-lang/crates.io-index","targets":[{"crate_types":["lib"],"doc":true,"doctest":true,"edition":"2018","kind":["lib"],"name":"is_debug","src_path":".cargo/registry/src/index.crates.io-1949cf8c6b5b557f/is_debug-1.1.0/lib.rs","test":true}],"version":"1.1.0"},{"authors":[],"categories":[],"default_run":null,"dependencies":[{"features":[],"kind":null,"name":"is_debug","optional":false,"registry":null,"rename":null,"req":"^1.1.0","source":"registry+https://github.com/rust-lang/crates.io-index","target":null,"uses_default_features":true}],"description":null,"documentation":null,"edition":"2024","features":{},"homepage":null,"id":"path+file://demo#0.1.0","keywords":[],"license":null,"license_file":null,"links":null,"manifest_path":"demo/Cargo.toml","metadata":null,"name":"rust-demo","publish":null,"readme":null,"repository":null,"rust_version":null,"source":null,"targets":[{"crate_types":["bin"],"doc":true,"doctest":false,"edition":"2024","kind":["bin"],"name":"rust-demo","src_path":"demo/src/main.rs","test":true}],"version":"0.1.0"}],"resolve":{"nodes":[{"dependencies":[],"deps":[],"features":["default","std"],"id":"registry+https://github.com/rust-lang/crates.io-index#is_debug@1.1.0"},{"dependencies":["registry+https://github.com/rust-lang/crates.io-index#is_debug@1.1.0"],"deps":[{"dep_kinds":[{"kind":null,"target":null}],"name":"is_debug","pkg":"registry+https://github.com/rust-lang/crates.io-index#is_debug@1.1.0"}],"features":[],"id":"path+file://demo#0.1.0"}],"root":"path+file://demo#0.1.0"},"target_directory":"demo/target","version":1,"workspace_default_members":["path+file://demo#0.1.0"],"workspace_members":["path+file://demo#0.1.0"],"workspace_root":"demo"}"#;

        let meta: cargo_metadata::Metadata = serde_json::from_str(metadata_json).unwrap();
        assert_eq!(meta.packages.len(), 2);
        assert!(meta.workspace_metadata.is_null());
    }
}
