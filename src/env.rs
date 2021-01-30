use crate::build::*;
use crate::channel::*;
use crate::err::SdResult;

use chrono::Local;
use std::env;
use std::process::Command;

use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct SystemEnv {
    map: HashMap<ShadowConst, ConstVal>,
}

const BUILD_OS: ShadowConst = "BUILD_OS";
const RUST_VERSION: ShadowConst = "RUST_VERSION";
const RUST_CHANNEL: ShadowConst = "RUST_CHANNEL";
const CARGO_VERSION: ShadowConst = "CARGO_VERSION";
const CARGO_TREE: ShadowConst = "CARGO_TREE";
// const CARGO_METADATA: ShadowConst = "CARGO_METADATA";
const PKG_VERSION: ShadowConst = "PKG_VERSION";
const PKG_VERSION_MAJOR: ShadowConst = "PKG_VERSION_MAJOR";
const PKG_VERSION_MINOR: ShadowConst = "PKG_VERSION_MINOR";
const PKG_VERSION_PATCH: ShadowConst = "PKG_VERSION_PATCH";
const PKG_VERSION_PRE: ShadowConst = "PKG_VERSION_PRE";

impl SystemEnv {
    fn init(&mut self, std_env: &HashMap<String, String>) -> SdResult<()> {
        let mut update_val = |c: ShadowConst, v: String| {
            if let Some(mut val) = self.map.get_mut(c) {
                val.t = ConstType::Str;
                val.v = v;
            }
        };
        if let Ok(out) = Command::new("rustup").arg("default").output() {
            update_val(
                RUST_CHANNEL,
                String::from_utf8(out.stdout)?.trim().to_string(),
            );
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
                let lines = filter_cargo_tree(input[index..].split('\n').collect());
                update_val(CARGO_TREE, lines);
            }
        }

        if let Ok(_out) = Command::new("cargo")
            .args(&["metadata", "--format-version", "1"])
            .output()
        {
            //TODO completed

            // update_val(
            //     CARGO_METADATA,
            //     String::from_utf8(out.stdout)?.trim().to_string(),
            // );
        }

        if let Some(v) = std_env.get("CARGO_PKG_VERSION") {
            update_val(PKG_VERSION, v.to_string());
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

        Ok(())
    }
}

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
fn filter_dep_source(input: &str) -> String {

    let (val, index) = if let Some(index) = input.find(" (/") {
        (" (* path)", index)
    } else if let Some(index) = input.find(" (registry ") {
        (" (* registry)", index)
    } else if let Some(index) = input.find(" (http") {
        (" (* git)", index)
    } else if let Some(index) = input.find(" (https") {
        (" (* git)", index)
    } else if let Some(index) = input.find(" (git") {
        (" (* git)", index)
    } else {
        ("", input.len())
    };
    format!("{}{}", input[..index].to_string(), val)
}

fn filter_cargo_tree(lines: Vec<&str>) -> String {
    let mut tree = "\n".to_string();
    for line in lines {
        let val = filter_dep_source(line);
        if tree.trim().is_empty() {
            tree.push_str(&val);
        } else {
            tree = format!("{}\n{}", tree, val);
        }
    }
    tree
}

pub fn new_system_env(std_env: &HashMap<String, String>) -> HashMap<ShadowConst, ConstVal> {
    let mut env = SystemEnv::default();
    env.map.insert(
        BUILD_OS,
        ConstVal {
            desc: "display build system os".to_string(),
            v: format!("{}-{}", env::consts::OS, env::consts::ARCH),
            t: ConstType::Str,
        },
    );

    env.map.insert(
        RUST_CHANNEL,
        ConstVal::new("display build system rust channel"),
    );
    env.map.insert(
        RUST_VERSION,
        ConstVal::new("display build system rust version"),
    );
    env.map.insert(
        CARGO_VERSION,
        ConstVal::new("display build system cargo version"),
    );

    env.map.insert(
        CARGO_TREE,
        ConstVal::new("display build cargo dependencies.It's used by rust version 1.44.0"),
    );

    // env.map.insert(
    //     CARGO_METADATA,
    //     ConstVal::new("display build cargo dependencies by metadata.It's use by exec command `cargo metadata`"),
    // );

    env.map.insert(
        PKG_VERSION,
        ConstVal::new("display build current project version"),
    );

    env.map.insert(
        PKG_VERSION_MAJOR,
        ConstVal::new("display build current project major version"),
    );
    env.map.insert(
        PKG_VERSION_MINOR,
        ConstVal::new("display build current project minor version"),
    );
    env.map.insert(
        PKG_VERSION_PATCH,
        ConstVal::new("display build current project patch version"),
    );
    env.map.insert(
        PKG_VERSION_PRE,
        ConstVal::new("display build current project preview version"),
    );

    if let Err(e) = env.init(std_env) {
        println!("{}", e.to_string());
    }
    env.map
}

#[derive(Default, Debug)]
pub struct Project {
    map: HashMap<ShadowConst, ConstVal>,
}

const PROJECT_NAME: ShadowConst = "PROJECT_NAME";
const BUILD_TIME: ShadowConst = "BUILD_TIME";
const BUILD_RUST_CHANNEL: ShadowConst = "BUILD_RUST_CHANNEL";

pub fn new_project(std_env: &HashMap<String, String>) -> HashMap<ShadowConst, ConstVal> {
    let mut project = Project::default();
    project.map.insert(
        BUILD_TIME,
        ConstVal {
            desc: "display project build time".to_string(),
            v: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            t: ConstType::Str,
        },
    );
    project.map.insert(
        BUILD_RUST_CHANNEL,
        ConstVal {
            desc: "display project build by rust channel [debug or release]".to_string(),
            v: build_channel().to_string(),
            t: ConstType::Str,
        },
    );
    project
        .map
        .insert(PROJECT_NAME, ConstVal::new("display project name"));

    if let (Some(v), Some(mut val)) = (
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
    use super::*;

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
}
