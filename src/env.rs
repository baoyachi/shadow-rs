use crate::build::*;
use crate::channel::*;
use crate::err::SdResult;

use chrono::Local;
use std::process::Command;
use std::{env, fs};

#[derive(Default, Debug)]
pub struct SystemEnv {
    pub const_msg: Vec<ConstMessage>,
}

const BUILD_OS: &str = "BUILD_OS";
const RUST_VERSION: &str = "RUST_VERSION";
const RUST_CHANNEL: &str = "RUST_CHANNEL";
const CARGO_VERSION: &str = "CARGO_VERSION";
const CARGO_LOCK: &str = "CARGO_LOCK";
// const CARGO_TREE: &str = "CARGO_TREE";

impl SystemEnv {
    pub fn new() -> SdResult<Self> {
        let mut vec = vec![];
        vec.push(ConstMessage {
            desc: "display build system os".to_string(),
            key: BUILD_OS.to_string(),
            val: format!("{}-{}", env::consts::OS, env::consts::ARCH),
            t: ConstType::Str,
        });
        if let Ok(out) = Command::new("rustup").arg("default").output() {
            vec.push(ConstMessage {
                desc: "display build system rust channel".to_string(),
                key: RUST_CHANNEL.to_string(),
                val: String::from_utf8(out.stdout)?.trim().to_string(),
                t: ConstType::Str,
            });
        }

        if let Ok(out) = Command::new("rustc").arg("-V").output() {
            vec.push(ConstMessage {
                desc: "display build system rust version".to_string(),
                key: RUST_VERSION.to_string(),
                val: String::from_utf8(out.stdout)?.trim().to_string(),
                t: ConstType::Str,
            });
        }

        if let Ok(out) = Command::new("cargo").arg("-V").output() {
            vec.push(ConstMessage {
                desc: "display build system cargo version".to_string(),
                key: CARGO_VERSION.to_string(),
                val: String::from_utf8(out.stdout)?.trim().to_string(),
                t: ConstType::Str,
            });
        }

        if let Ok(v) = fs::read_to_string("Cargo.lock")
        //TODO this need fix cargo path;
        {
            vec.push(ConstMessage {
                desc: "display build project dependence cargo lock detail".to_string(),
                key: CARGO_LOCK.to_string(),
                val: format!("{:?}", v),
                t: ConstType::Str,
            });
        }
        Ok(SystemEnv { const_msg: vec })
    }
}

#[derive(Default, Debug)]
pub struct Project {
    pub const_msg: Vec<ConstMessage>,
}

const PROJECT_NAME: &str = "PROJECT_NAME";
const BUILD_TIME: &str = "BUILD_TIME";
const BUILD_RUST_CHANNEL: &str = "BUILD_RUST_CHANNEL";

impl Project {
    pub fn new() -> Project {
        let mut vec = vec![];
        if let Some(v) = option_env!("CARGO_PKG_NAME") {
            vec.push(ConstMessage {
                desc: "display project name".to_string(),
                key: PROJECT_NAME.to_string(),
                val: v.to_string(),
                t: ConstType::Str,
            });
        }

        vec.push(ConstMessage {
            desc: "display project build time".to_string(),
            key: BUILD_TIME.to_string(),
            val: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), //TODO need change by config time_type
            t: ConstType::Str,
        });

        vec.push(ConstMessage {
            desc: "display project build by rust channel [debug or release]".to_string(),
            key: BUILD_RUST_CHANNEL.to_string(),
            val: build_channel().to_string(),
            t: ConstType::Str,
        });

        Project { const_msg: vec }
    }
}
