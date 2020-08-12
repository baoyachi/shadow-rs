use chrono::Local;
use crate::channel::*;
use std::{env, fs};
use std::process::Command;

#[derive(Default, Debug)]
struct Shadow {
    project: Project
}

#[derive(Default, Debug)]
pub struct Environment {
    build_os: String,
    rust_version: String,
    rust_channel: String,
    cargo_version: String,
    cargo_tree: String,
    cargo_lock: String,
}

impl Environment {
    pub fn new() -> Environment {
        let build_os = format!("{}-{}", env::consts::OS, env::consts::ARCH);
        let rustup = Command::new("rustup")
            .arg("default")
            .status()
            .expect("failed to execute process");

        let rust_version = Command::new("rustc")
            .arg("-V")
            .status()
            .expect("failed to execute process");

        let cargo_version = Command::new("cargo")
            .arg("-V")
            .status()
            .expect("failed to execute process");

        let cargo_lock = fs::read_to_string("Cargo.lock").unwrap();

        Environment {
            build_os,
            rust_version: rust_version.to_string(),
            rust_channel: rustup.to_string(),
            cargo_version: cargo_version.to_string(),
            cargo_tree: "".to_string(),
            cargo_lock: cargo_lock.to_string(),
        }
    }
}


#[derive(Default, Debug)]
struct Project {
    pkg_name: String,
    build_time: String,
    build_channel: BuildChannel,
}

impl Project {
    fn get_project(&mut self) {
        self.pkg_name = env!("CARGO_PKG_NAME").into();
        self.build_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        self.build_channel = build_channel()
    }
}


impl Shadow {}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment() {
        println!("{:?}",Environment::new());
    }


    #[test]
    fn test_project() {
        let mut project = Project::default();
        project.get_project();
        println!("{:?}", project);
    }
}