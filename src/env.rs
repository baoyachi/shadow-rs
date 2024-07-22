use crate::build::*;
use crate::date_time::now_date_time;
use crate::env::dep_source_replace::filter_cargo_tree;
use crate::err::SdResult;
use crate::Format;
use is_debug::build_channel;
use std::collections::BTreeMap;
use std::env;
use std::process::Command;

#[derive(Default, Debug)]
pub struct SystemEnv {
    map: BTreeMap<ShadowConst, ConstVal>,
}

const BUILD_OS_DOC: &str = r#"
Operating system and architecture on which the project was build.
The format of this variable is always `os-arch`,
where `os` is the operating system name as returned by [`std::env::consts::OS`],
and `arch` is the computer architecture as returned by [`std::env::consts::ARCH`]."#;
pub const BUILD_OS: ShadowConst = "BUILD_OS";

const RUST_VERSION_DOC: &str = r#"
Rust version with which the project was built.
The version always uses the canonical Rust version format,
and is therefore identical to the output of the build toolchain's `rustc --version`."#;
pub const RUST_VERSION: ShadowConst = "RUST_VERSION";

const RUST_CHANNEL_DOC: &str = r#"
The [Rustup toolchain](https://rust-lang.github.io/rustup/concepts/toolchains.html) with which the project was built.
Note that as per Rustup toolchain format, this variable may or may not contain host and date information,
but it will always contain [channel](https://rust-lang.github.io/rustup/concepts/channels.html) information (stable, beta or nightly)."#;
pub const RUST_CHANNEL: ShadowConst = "RUST_CHANNEL";

pub const CARGO_METADATA: ShadowConst = "CARGO_METADATA";
const CARGO_METADATA_DOC: ShadowConst = r#"
The information about the workspace members and resolved dependencies of the current package.
See the [cargo_metadata](https://crates.io/crates/cargo_metadata) crate for a Rust API for reading the metadata."#;

const CARGO_VERSION_DOC: &str = r#"
The cargo version which which the project was built, as output by `cargo --version`."#;
pub const CARGO_VERSION: ShadowConst = "CARGO_VERSION";

const CARGO_TREE_DOC: &str = r#"
The dependency tree of the project, as output by `cargo tree`.
Note that this variable may contain local file system paths for path dependencies, and may therefore contain sensitive information and not be reproducible."#;
pub const CARGO_TREE: ShadowConst = "CARGO_TREE";

const BUILD_TARGET_DOC: &str = r#"
The [target](https://doc.rust-lang.org/rustc/targets/index.html) for this build.
This is possibly distinct from the host target during build, in which case this project build was created via cross-compilation."#;
pub const BUILD_TARGET: ShadowConst = "BUILD_TARGET";

const BUILD_TARGET_ARCH_DOC: &str = r#"
The architecture of the target for this build. This is the "architecture" part of the [`BUILD_TARGET`] constant."#;
pub const BUILD_TARGET_ARCH: ShadowConst = "BUILD_TARGET_ARCH";

const CARGO_MANIFEST_DIR_DOC: &str = r#"
The directory of the Cargo.toml manifest file of the project during build.
Note that this variable will contain a full local file system path, and will therefore contain sensitive information and not be reproducible."#;
pub const CARGO_MANIFEST_DIR: ShadowConst = "CARGO_MANIFEST_DIR";

const PKG_VERSION_DOC: &str = r#"
The project's full version string, as determined by the Cargo.toml manifest."#;
pub const PKG_VERSION: ShadowConst = "PKG_VERSION";

const PKG_DESCRIPTION_DOC: &str = r#"
The project's description, as determined by the Cargo.toml manifest."#;
pub const PKG_DESCRIPTION: ShadowConst = "PKG_DESCRIPTION";

const PKG_VERSION_MAJOR_DOC: &str = r#"
The project's semver major version, as determined by the Cargo.toml manifest."#;
pub const PKG_VERSION_MAJOR: ShadowConst = "PKG_VERSION_MAJOR";

const PKG_VERSION_MINOR_DOC: &str = r#"
The project's semver minor version, as determined by the Cargo.toml manifest."#;
pub const PKG_VERSION_MINOR: ShadowConst = "PKG_VERSION_MINOR";

const PKG_VERSION_PATCH_DOC: &str = r#"
The project's semver patch version, as determined by the Cargo.toml manifest."#;
pub const PKG_VERSION_PATCH: ShadowConst = "PKG_VERSION_PATCH";

const PKG_VERSION_PRE_DOC: &str = r#"
The project's semver pre-release version, as determined by the Cargo.toml manifest."#;
pub const PKG_VERSION_PRE: ShadowConst = "PKG_VERSION_PRE";

impl SystemEnv {
    fn init(&mut self, std_env: &BTreeMap<String, String>) -> SdResult<()> {
        let mut update_val = |c: ShadowConst, v: String| {
            if let Some(val) = self.map.get_mut(c) {
                val.v = v;
            }
        };

        if let Some(v) = std_env.get("RUSTUP_TOOLCHAIN") {
            update_val(RUST_CHANNEL, v.to_string());
        }

        if let Ok(out) = Command::new("rustc").arg("-V").output() {
            update_val(
                RUST_VERSION,
                String::from_utf8(out.stdout)?.trim().to_string(),
            );
        }

        if let Ok(out) = Command::new("cargo").arg("-V").output() {
            update_val(
                CARGO_VERSION,
                String::from_utf8(out.stdout)?.trim().to_string(),
            );
        }

        if let Ok(out) = Command::new("cargo").arg("tree").output() {
            let input = String::from_utf8(out.stdout)?;
            if let Some(index) = input.find('\n') {
                let lines =
                    filter_cargo_tree(input.get(index..).unwrap_or_default().split('\n').collect());
                update_val(CARGO_TREE, lines);
            }
        }

        if let Ok(out) = Command::new("cargo")
            .args(["metadata", "--format-version", "1"])
            .output()
        {
            update_val(
                CARGO_METADATA,
                String::from_utf8(out.stdout)?.trim().to_string(),
            );
        }

        if let Some(v) = std_env.get("TARGET") {
            update_val(BUILD_TARGET, v.to_string());
        }

        if let Some(v) = std_env.get("CARGO_CFG_TARGET_ARCH") {
            update_val(BUILD_TARGET_ARCH, v.to_string());
        }

        if let Some(v) = std_env.get("CARGO_PKG_VERSION") {
            update_val(PKG_VERSION, v.to_string());
        }

        if let Some(v) = std_env.get("CARGO_PKG_DESCRIPTION") {
            update_val(PKG_DESCRIPTION, v.to_string());
        }

        if let Some(v) = std_env.get("CARGO_PKG_VERSION_MAJOR") {
            update_val(PKG_VERSION_MAJOR, v.to_string());
        }

        if let Some(v) = std_env.get("CARGO_PKG_VERSION_MINOR") {
            update_val(PKG_VERSION_MINOR, v.to_string());
        }
        if let Some(v) = std_env.get("CARGO_PKG_VERSION_PATCH") {
            update_val(PKG_VERSION_PATCH, v.to_string());
        }
        if let Some(v) = std_env.get("CARGO_PKG_VERSION_PRE") {
            update_val(PKG_VERSION_PRE, v.to_string());
        }
        if let Some(v) = std_env.get("CARGO_MANIFEST_DIR") {
            update_val(CARGO_MANIFEST_DIR, v.to_string());
        }

        Ok(())
    }
}

mod dep_source_replace {
    use std::fs;

    fn path_exists(path: &str) -> bool {
        fs::metadata(path).is_ok()
    }

    const DEP_REPLACE_NONE: &str = "";
    const DEP_REPLACE_PATH: &str = " (* path)";
    const DEP_REPLACE_GIT: &str = " (* git)";
    const DEP_REPLACE_REGISTRY: &str = " (* registry)";

    /// filter cargo tree dependencies source
    ///
    /// Why do this?
    ///
    /// Sometimes, the private registry or private git url that our cargo relies on will carry this information
    /// with the cargo tree command output we use. In order to protect the privacy of dependence, we need to shield it.
    ///
    /// This can protect us from the security issues we rely on environmental information.
    ///
    /// I think it is very necessary.So we need to do fuzzy replacement of dependent output.
    ///
    /// for examples:
    ///
    /// - dep by git: shadow-rs = { git = "https://github.com/baoyachi/shadow-rs", branch="master" }
    /// - dep by registry: shadow-rs = { version = "0.5.23",registry="private-crates" }
    /// - dep by path: shadow-rs = { path = "/Users/baoyachi/shadow-rs" }
    ///
    ///  before exec: cargo tree output by difference dependencies source:
    ///
    /// - git: └── shadow-rs v0.5.23 (https://github.com/baoyachi/shadow-rs?branch=master#eb712990)
    /// - registry: └── shadow-rs v0.5.23 (registry ssh://git@github.com/baoyachi/shadow-rs.git)
    /// - path: └── shadow-rs v0.5.23 ((/Users/baoyachi/shadow-rs))
    ///
    /// after filter dependencies source
    ///
    /// - git: └── shadow-rs v0.5.23 (* git)
    /// - registry: └── shadow-rs v0.5.23 (* registry)
    /// - path: └── shadow-rs v0.5.23 (* path)
    ///
    pub fn filter_dep_source(input: &str) -> String {
        let (val, index) = if let Some(index) = input.find(" (/") {
            (DEP_REPLACE_PATH, index)
        } else if let Some(index) = input.find(" (registry ") {
            (DEP_REPLACE_REGISTRY, index)
        } else if let Some(index) = input.find(" (http") {
            (DEP_REPLACE_GIT, index)
        } else if let Some(index) = input.find(" (https") {
            (DEP_REPLACE_GIT, index)
        } else if let Some(index) = input.find(" (ssh") {
            (DEP_REPLACE_GIT, index)
        } else if let (Some(start), Some(end)) = (input.find(" ("), input.find(')')) {
            let path = input.get(start + 2..end).unwrap_or_default().trim();
            if path_exists(path) {
                (DEP_REPLACE_PATH, start)
            } else {
                (DEP_REPLACE_NONE, input.len())
            }
        } else {
            (DEP_REPLACE_NONE, input.len())
        };
        format!("{}{}", &input.get(..index).unwrap_or_default(), val)
    }

    pub fn filter_cargo_tree(lines: Vec<&str>) -> String {
        let mut tree = "\n".to_string();
        for line in lines {
            let val = filter_dep_source(line);
            if tree.trim().is_empty() {
                tree.push_str(&val);
            } else {
                tree = format!("{tree}\n{val}");
            }
        }
        tree
    }
}

/// Create all `shadow-rs` constants which are determined by the build environment.
/// The data for these constants is provided by the `std_env` argument.
pub(crate) fn new_system_env(
    std_env: &BTreeMap<String, String>,
) -> BTreeMap<ShadowConst, ConstVal> {
    let mut env = SystemEnv::default();
    env.map.insert(
        BUILD_OS,
        ConstVal {
            desc: BUILD_OS_DOC.to_string(),
            v: format!("{}-{}", env::consts::OS, env::consts::ARCH),
            t: ConstType::Str,
        },
    );

    env.map
        .insert(RUST_CHANNEL, ConstVal::new(RUST_CHANNEL_DOC));
    env.map
        .insert(CARGO_METADATA, ConstVal::new_slice(CARGO_METADATA_DOC));
    env.map
        .insert(RUST_VERSION, ConstVal::new(RUST_VERSION_DOC));
    env.map
        .insert(CARGO_VERSION, ConstVal::new(CARGO_VERSION_DOC));

    env.map.insert(CARGO_TREE, ConstVal::new(CARGO_TREE_DOC));

    // env.map.insert(
    //     CARGO_METADATA,
    //     ConstVal::new("display build cargo dependencies by metadata.It's use by exec command `cargo metadata`"),
    // );

    env.map
        .insert(BUILD_TARGET, ConstVal::new(BUILD_TARGET_DOC));

    env.map
        .insert(BUILD_TARGET_ARCH, ConstVal::new(BUILD_TARGET_ARCH_DOC));

    env.map.insert(PKG_VERSION, ConstVal::new(PKG_VERSION_DOC));

    env.map
        .insert(PKG_DESCRIPTION, ConstVal::new(PKG_DESCRIPTION_DOC));

    env.map
        .insert(PKG_VERSION_MAJOR, ConstVal::new(PKG_VERSION_MAJOR_DOC));
    env.map
        .insert(PKG_VERSION_MINOR, ConstVal::new(PKG_VERSION_MINOR_DOC));
    env.map
        .insert(PKG_VERSION_PATCH, ConstVal::new(PKG_VERSION_PATCH_DOC));
    env.map
        .insert(PKG_VERSION_PRE, ConstVal::new(PKG_VERSION_PRE_DOC));
    env.map
        .insert(CARGO_MANIFEST_DIR, ConstVal::new(CARGO_MANIFEST_DIR_DOC));

    if let Err(e) = env.init(std_env) {
        println!("{e}");
    }
    env.map
}

#[derive(Default, Debug)]
pub struct Project {
    map: BTreeMap<ShadowConst, ConstVal>,
}

const PROJECT_NAME_DOC: &str = r#"
The project name, as determined by the Cargo.toml manifest."#;
const PROJECT_NAME: ShadowConst = "PROJECT_NAME";

const BUILD_TIME_DOC: &str = r#"
The project build time, formatted in modified ISO 8601 format (`YYYY-MM-DD HH-MM ±hh-mm` where hh-mm is the offset from UTC)."#;
const BUILD_TIME: ShadowConst = "BUILD_TIME";

const BUILD_TIME_2822_DOC: &str = r#"
The project build time, formatted according to [RFC 2822](https://datatracker.ietf.org/doc/html/rfc2822#section-3.3) (e.g. HTTP Headers)."#;
const BUILD_TIME_2822: ShadowConst = "BUILD_TIME_2822";

const BUILD_TIME_3339_DOC: &str = r#"
The project build time, formatted according to [RFC 3339 and ISO 8601](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6)."#;
const BUILD_TIME_3339: ShadowConst = "BUILD_TIME_3339";

const BUILD_RUST_CHANNEL_DOC: &str = r#"
The debug configuration with which the project was built.
Note that this is not the Rust channel, but either `debug` or `release`, depending on whether debug assertions were enabled in the build or not. "#;
const BUILD_RUST_CHANNEL: ShadowConst = "BUILD_RUST_CHANNEL";

pub(crate) fn build_time(project: &mut Project) {
    // Enable reproducible builds: https://reproducible-builds.org/docs/source-date-epoch/
    let time = now_date_time();
    project.map.insert(
        BUILD_TIME,
        ConstVal {
            desc: BUILD_TIME_DOC.to_string(),
            v: time.human_format(),
            t: ConstType::Str,
        },
    );
    project.map.insert(
        BUILD_TIME_2822,
        ConstVal {
            desc: BUILD_TIME_2822_DOC.to_string(),
            v: time.to_rfc2822(),
            t: ConstType::Str,
        },
    );

    project.map.insert(
        BUILD_TIME_3339,
        ConstVal {
            desc: BUILD_TIME_3339_DOC.to_string(),
            v: time.to_rfc3339(),
            t: ConstType::Str,
        },
    );
}

pub(crate) fn new_project(std_env: &BTreeMap<String, String>) -> BTreeMap<ShadowConst, ConstVal> {
    let mut project = Project::default();
    build_time(&mut project);
    project.map.insert(
        BUILD_RUST_CHANNEL,
        ConstVal {
            desc: BUILD_RUST_CHANNEL_DOC.to_string(),
            v: build_channel().to_string(),
            t: ConstType::Str,
        },
    );
    project
        .map
        .insert(PROJECT_NAME, ConstVal::new(PROJECT_NAME_DOC));

    if let (Some(v), Some(val)) = (
        std_env.get("CARGO_PKG_NAME"),
        project.map.get_mut(PROJECT_NAME),
    ) {
        val.t = ConstType::Str;
        val.v = v.to_string();
    }

    project.map
}

#[cfg(test)]
mod tests {
    use crate::env::dep_source_replace::filter_dep_source;

    #[test]
    fn test_filter_dep_source_none() {
        let input = "shadow-rs v0.5.23";
        let ret = filter_dep_source(input);
        assert_eq!(input, ret)
    }

    #[test]
    fn test_filter_dep_source_multi() {
        let input = "shadow-rs v0.5.23 (*)";
        let ret = filter_dep_source(input);
        assert_eq!(input, ret)
    }

    #[test]
    fn test_filter_dep_source_path() {
        let input = "shadow-rs v0.5.23 (/Users/baoyachi/shadow-rs)";
        let ret = filter_dep_source(input);
        assert_eq!("shadow-rs v0.5.23 (* path)", ret)
    }

    #[test]
    fn test_filter_dep_source_registry() {
        let input = "shadow-rs v0.5.23 (registry `ssh://git@github.com/baoyachi/shadow-rs.git`)";
        let ret = filter_dep_source(input);
        assert_eq!("shadow-rs v0.5.23 (* registry)", ret)
    }

    #[test]
    fn test_filter_dep_source_git_https() {
        let input = "shadow-rs v0.5.23 (https://github.com/baoyachi/shadow-rs#13572c90)";
        let ret = filter_dep_source(input);
        assert_eq!("shadow-rs v0.5.23 (* git)", ret)
    }

    #[test]
    fn test_filter_dep_source_git_http() {
        let input = "shadow-rs v0.5.23 (http://github.com/baoyachi/shadow-rs#13572c90)";
        let ret = filter_dep_source(input);
        assert_eq!("shadow-rs v0.5.23 (* git)", ret)
    }

    #[test]
    fn test_filter_dep_source_git() {
        let input = "shadow-rs v0.5.23 (ssh://git@github.com/baoyachi/shadow-rs)";
        let ret = filter_dep_source(input);
        assert_eq!("shadow-rs v0.5.23 (* git)", ret)
    }

    #[test]
    fn test_filter_dep_windows_path() {
        let input = r"shadow-rs v0.5.23 (FD:\a\shadow-rs\shadow-rs)";
        let ret = filter_dep_source(input);
        assert_eq!(input, ret)
    }
}
