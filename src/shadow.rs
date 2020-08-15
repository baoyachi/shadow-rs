use chrono::Local;
use crate::channel::*;
use std::{env, fs};
use crate::err::SdResult;
use std::process::{Command, Stdio};
use crate::git::Git;


#[derive(Default, Debug)]
pub struct Shadow {
    project: Project,
    sys_env: SystemEnv,
    git: Git,
}

impl Shadow {
    pub fn new() -> Shadow {
        Shadow {
            project: Project::new(),
            sys_env: SystemEnv::new().unwrap(),
            git: Git::new("./"),
        }
    }
}

#[derive(Default, Debug)]
pub struct SystemEnv {
    build_os: String,
    rust_version: String,
    rust_channel: String,
    cargo_version: String,
    cargo_tree: String,
    cargo_lock: String,
}

impl SystemEnv {
    pub fn new() -> SdResult<SystemEnv> {
        let build_os = format!("{}-{}", env::consts::OS, env::consts::ARCH);
        let rustup_output = Command::new("rustup")
            .arg("default")
            .output().unwrap();
        let rustup = String::from_utf8(rustup_output.stdout).unwrap();

        let rust_version_output = Command::new("rustc")
            .arg("-V")
            .output().unwrap();
        let rust_version = String::from_utf8(rust_version_output.stdout).unwrap();

        let cargo_version_output = Command::new("cargo")
            .arg("-V")
            .output().unwrap();

        let cargo_version = String::from_utf8(cargo_version_output.stdout)?;

        let cargo_lock = fs::read_to_string("Cargo.lock").unwrap();

        Ok(SystemEnv {
            build_os,
            rust_version: rust_version.trim().to_string(),
            rust_channel: rustup.trim().to_string(),
            cargo_version: cargo_version.trim().to_string(),
            cargo_tree: "".to_string(),
            cargo_lock: cargo_lock.to_string(),
        })
    }
}


#[derive(Default, Debug)]
pub struct Project {
    pkg_name: String,
    build_time: String,
    build_channel: BuildChannel,
}

impl Project {
    fn new() -> Project {
        let pkg_name = env!("CARGO_PKG_NAME").to_string();
        let build_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let build_channel = build_channel();
        Project {
            pkg_name,
            build_time,
            build_channel,
        }
    }
}


impl Shadow {}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment() {
        println!("{:?}", SystemEnv::new().unwrap());
    }
}