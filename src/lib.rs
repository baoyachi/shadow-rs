#![doc(html_logo_url = "https://raw.githubusercontent.com/baoyachi/shadow-rs/master/shadow-rs.png")]
//! `shadow-rs` :build-time information stored in your rust project.(binary,lib,cdylib,dylib)
//!
//! It's allows you to recall properties of the build process and environment at runtime, including:
//!
//! `Cargo.toml` project version
//! * Dependency information
//! * The Git commit that produced the build artifact (binary)
//! * What version of the rust toolchain was used in compilation
//! * The build variant, e.g. `debug` or `release`
//! * (And more)
//!
//! You can use this tool to check in production exactly where a binary came from and how it was built.
//!
//! # Full Examples
//! * Check out the [example_shadow](https://github.com/baoyachi/shadow-rs/tree/master/example_shadow) for a simple demonstration of how `shadow-rs` might be used to provide build-time information at run-time.
//! * Check out the [example_shadow_hook](https://github.com/baoyachi/shadow-rs/tree/master/example_shadow_hook) for a simple demonstration of how `shadow-rs` might be used to provide build-time information at run-time,and add custom hook.
//!
//! ## Built in function
//! * Check out the [examples](https://github.com/baoyachi/shadow-rs/tree/master/examples) for a simple demonstration of how `shadow-rs` might be used to provide build in function.
//!
//! # Example
//!
//! ```
//! pub const PKG_VERSION :&str = "1.3.8-beta3";
//! pub const PKG_VERSION_MAJOR :&str = "1";
//! pub const PKG_VERSION_MINOR :&str = "3";
//! pub const PKG_VERSION_PATCH :&str = "8";
//! pub const PKG_VERSION_PRE :&str = "beta3";
//! pub const RUST_VERSION :&str = "rustc 1.45.0 (5c1f21c3b 2020-07-13)";
//! pub const BUILD_RUST_CHANNEL :&str = "debug";
//! pub const COMMIT_AUTHOR :&str = "baoyachi";
//! pub const BUILD_TIME :&str = "2020-08-16 13:48:52";
//! pub const BUILD_TIME_2822 :&str = "Thu, 24 Jun 2021 21:44:14 +0800";
//! pub const BUILD_TIME_3339 :&str = "2021-06-24T15:53:55+08:00";
//! pub const COMMIT_DATE :&str = "2021-08-04 12:34:03 +00:00";
//! pub const COMMIT_DATE_2822 :&str = "Thu, 24 Jun 2021 21:44:14 +0800";
//! pub const COMMIT_DATE_3339 :&str = "2021-06-24T21:44:14.473058+08:00";
//! pub const COMMIT_EMAIL :&str = "xxx@gmail.com";
//! pub const PROJECT_NAME :&str = "shadow-rs";
//! pub const RUST_CHANNEL :&str = "stable-x86_64-apple-darwin (default)";
//! pub const BRANCH :&str = "master";
//! pub const CARGO_LOCK :&str = r#"
//! ├── chrono v0.4.19
//! │   ├── libc v0.2.80
//! │   ├── num-integer v0.1.44
//! │   │   └── num-traits v0.2.14
//! │   │       [build-dependencies]
//! │   │       └── autocfg v1.0.1
//! │   ├── num-traits v0.2.14 (*)
//! │   └── time v0.1.44
//! │       └── libc v0.2.80
//! └── git2 v0.13.12
//! ├── log v0.4.11
//! │   └── cfg-if v0.1.10
//! └── url v2.2.0
//! ├── form_urlencoded v1.0.0
//! │   └── percent-encoding v2.1.0
//! └── percent-encoding v2.1.0"#;
//! pub const CARGO_VERSION :&str = "cargo 1.45.0 (744bd1fbb 2020-06-15)";
//! pub const BUILD_OS :&str = "macos-x86_64";
//! pub const COMMIT_HASH :&str = "386741540d73c194a3028b96b92fdeb53ca2788a";
//! pub const GIT_CLEAN :bool = true;
//! pub const GIT_STATUS_FILE :&str = "* src/lib.rs (dirty)";
//! ```
//! # Setup Guide
//!
//! ### 1) Modify `Cargo.toml` fields
//! Modify your `Cargo.toml` like so:
//!
//! ```toml
//! [package]
//! build = "build.rs"
//!
//! [dependencies]
//! shadow-rs = "{latest version}"
//!
//! [build-dependencies]
//! shadow-rs = "{latest version}"
//! ```
#![cfg_attr(feature = "document-features", doc = document_features::document_features!())]
//!
//! ### 2) Create `build.rs` file
//! Now in the root of your project (same directory as `Cargo.toml`) add a file `build.rs`:
//!
//! ```ignore
//! fn main() -> shadow_rs::SdResult<()> {
//!    shadow_rs::new()
//! }
//! ```
//!
//! ### 3) Integrate Shadow
//! In your rust file (e.g. `*.rs`):
//!
//! ```ignore
//! use shadow_rs::shadow;
//!
//! shadow!(build);
//! ```
//! **Notice that the `shadow!` macro is provided the identifier `build`.  You can now use this identifier to access build-time information.**
//!
//! ### 4) Done. Use Shadow.
//! Then you can use const that's shadow build it(main.rs).
//!
//! The `build` mod just we use `shadow!(build)` generated.
//!
//! ```ignore
//! fn main(){
//!    println!("debug:{}", shadow_rs::is_debug()); // check if this is a debug build. e.g 'true/false'
//!    println!("branch:{}", shadow_rs::branch()); // get current project branch. e.g 'master/develop'
//!    println!("tag:{}", shadow_rs::tag()); // get current project tag. e.g 'v1.3.5'
//!    println!("git_clean:{}", shadow_rs::git_clean()); // get current project clean. e.g 'true/false'
//!    println!("git_status_file:{}", shadow_rs::git_status_file()); // get current project statue file. e.g '  * examples/builtin_fn.rs (dirty)'
//!
//!    println!("{}",build::VERSION); //print version const
//!    println!("{}",build::CLAP_LONG_VERSION); //print CLAP_LONG_VERSION const
//!    println!("{}",build::BRANCH); //master
//!    println!("{}",build::SHORT_COMMIT);//8405e28e
//!    println!("{}",build::COMMIT_HASH);//8405e28e64080a09525a6cf1b07c22fcaf71a5c5
//!    println!("{}",build::COMMIT_DATE);//2021-08-04 12:34:03 +00:00
//!    println!("{}",build::COMMIT_AUTHOR);//baoyachi
//!    println!("{}",build::COMMIT_EMAIL);//xxx@gmail.com
//!
//!    println!("{}",build::BUILD_OS);//macos-x86_64
//!    println!("{}",build::RUST_VERSION);//rustc 1.45.0 (5c1f21c3b 2020-07-13)
//!    println!("{}",build::RUST_CHANNEL);//stable-x86_64-apple-darwin (default)
//!    println!("{}",build::CARGO_VERSION);//cargo 1.45.0 (744bd1fbb 2020-06-15)
//!    println!("{}",build::PKG_VERSION);//0.3.13
//!    println!("{}",build::CARGO_TREE); //like command:cargo tree
//!    println!("{}",build::CARGO_MANIFEST_DIR); // /User/baoyachi/shadow-rs/ |
//!
//!    println!("{}",build::PROJECT_NAME);//shadow-rs
//!    println!("{}",build::BUILD_TIME);//2020-08-16 14:50:25
//!    println!("{}",build::BUILD_RUST_CHANNEL);//debug
//!    println!("{}",build::GIT_CLEAN);//false
//!    println!("{}",build::GIT_STATUS_FILE);//* src/lib.rs (dirty)
//! }
//!```
//!
//! ## Clap example
//! And you can also use `shadow-rs` with [`clap`](https://github.com/baoyachi/shadow-rs/blob/master/example_shadow/src/main.rs).
//!
//! For the user guide and further documentation, please read
//! [The shadow-rs document](https://github.com/baoyachi/shadow-rs).
//!

mod build;
mod ci;
mod date_time;
mod env;
mod err;
mod gen_const;
mod git;

/// Get current project build mode.
///
/// It's very useful. Debug mode is usually used for debugging information.
/// For example, log printing, environment variable switch.
///
/// The default value is `true`.
///
/// If we compile with `cargo build --release`. It's return value is `false`.
pub use is_debug::*;

use build::*;
use env::*;

use git::*;

use crate::ci::CiType;
pub use crate::date_time::DateTime;
pub use const_format::*;
use std::collections::BTreeMap;
use std::env as std_env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::gen_const::{
    clap_long_version_branch_const, clap_long_version_tag_const, clap_version_branch_const,
    clap_version_tag_const, version_branch_const, version_tag_const, BUILD_CONST_CLAP_LONG_VERSION,
    BUILD_CONST_VERSION,
};
pub use err::{SdResult, ShadowError};
pub use git::{branch, git_clean, git_status_file, tag};

pub trait Format {
    fn human_format(&self) -> String;
}

const SHADOW_RS: &str = "shadow.rs";

/// Add a mod in project with `$build_mod`.
///
/// You can use `shadow!(build_shadow)`. Then shadow-rs can help you add a mod with name `build_shadow`.
/// Next, use mod with name `build_shadow`,and also use const like:`build_shadow::BRANCH`.
///
/// Normally, you just config `shadow!(build)`.It looks more concise.
#[macro_export]
macro_rules! shadow {
    ($build_mod:ident) => {
        pub mod $build_mod {
            include!(concat!(env!("OUT_DIR"), "/shadow.rs"));
        }
    };
}

/// It's shadow-rs Initialization entry.
///
/// In build.rs `main()` function call for this function.
///
/// # Examples
///
/// ```ignore
/// fn main() -> shadow_rs::SdResult<()> {
///    shadow_rs::new()
/// }
/// ```
pub fn new() -> SdResult<()> {
    Shadow::build()?;
    Ok(())
}

/// It's shadow-rs Initialization entry with add custom hook.
///
/// In build.rs `main()` function call for this function.
///
/// # Examples
///
/// ```ignore
/// fn main() -> shadow_rs::SdResult<()> {
///    shadow_rs::new_hook(append_write_const)
/// }
///
/// fn append_write_const(mut file: &File) -> SdResult<()> {
///    let foo: &str = r#"pub const foo: &str = "foo";"#;
///    writeln!(file, "{}", foo)?;
///    Ok(())
/// }
///
/// ```
pub fn new_hook<F>(f: F) -> SdResult<()>
where
    F: FnOnce(&File) -> SdResult<()>,
{
    let shadow = Shadow::build()?;
    shadow.hook(f)
}

/// get std::env:vars
pub fn get_std_env() -> BTreeMap<String, String> {
    let mut env_map = BTreeMap::new();
    for (k, v) in std_env::vars() {
        env_map.insert(k, v);
    }
    env_map
}

#[derive(Debug)]
pub struct Shadow {
    pub f: File,
    pub map: BTreeMap<ShadowConst, ConstVal>,
    pub std_env: BTreeMap<String, String>,
}

impl Shadow {
    pub fn hook<F>(&self, f: F) -> SdResult<()>
    where
        F: FnOnce(&File) -> SdResult<()>,
    {
        let desc = r#"/// Below code generated by project custom from by build.rs"#;
        writeln!(&self.f, "\n{desc}\n")?;
        f(&self.f)?;
        Ok(())
    }

    /// try get current ci env
    fn try_ci(&self) -> CiType {
        if let Some(c) = self.std_env.get("GITLAB_CI") {
            if c == "true" {
                return CiType::Gitlab;
            }
        }

        if let Some(c) = self.std_env.get("GITHUB_ACTIONS") {
            if c == "true" {
                return CiType::Github;
            }
        }

        //TODO completed [travis,jenkins] env

        CiType::None
    }

    pub fn build() -> SdResult<Shadow> {
        let src_path = std::env::var("CARGO_MANIFEST_DIR")?;
        let out_path = std::env::var("OUT_DIR")?;
        Self::build_inner(src_path, out_path)
    }

    fn build_inner(src_path: String, out_path: String) -> SdResult<Shadow> {
        let out = {
            let path = Path::new(out_path.as_str());
            if !out_path.ends_with('/') {
                path.join(format!("{out_path}/{SHADOW_RS}"))
            } else {
                path.join(SHADOW_RS)
            }
        };

        let mut shadow = Shadow {
            f: File::create(out)?,
            map: Default::default(),
            std_env: Default::default(),
        };
        shadow.std_env = get_std_env();

        let ci_type = shadow.try_ci();
        let src_path = Path::new(src_path.as_str());

        let mut map = new_git(src_path, ci_type, &shadow.std_env);
        for (k, v) in new_project(&shadow.std_env) {
            map.insert(k, v);
        }
        for (k, v) in new_system_env(&shadow.std_env) {
            map.insert(k, v);
        }
        shadow.map = map;

        shadow.write_all()?;

        Ok(shadow)
    }

    fn write_all(&mut self) -> SdResult<()> {
        self.gen_header()?;

        self.gen_const()?;

        //write version function
        let gen_version = self.gen_version()?;

        self.gen_build_in(gen_version)?;

        Ok(())
    }

    pub fn cargo_rerun_if_env_changed(&self) {
        for k in self.std_env.keys() {
            println!("cargo:rerun-if-env-changed={k}");
        }
    }

    pub fn cargo_rerun_env_inject(&self, env: &[&str]) {
        for k in env {
            println!("cargo:rerun-if-env-changed={}", *k);
        }
    }

    fn gen_const(&mut self) -> SdResult<()> {
        for (k, v) in self.map.clone() {
            println!("cargo:rerun-if-env-changed={k}");
            self.write_const(k, v)?;
        }
        Ok(())
    }

    fn gen_header(&self) -> SdResult<()> {
        let desc = format!(
            r#"// Code generated by shadow-rs generator. DO NOT EDIT.
// Author by:https://www.github.com/baoyachi
// The build script repository:https://github.com/baoyachi/shadow-rs
// Create time by:{}"#,
            DateTime::now().human_format()
        );
        writeln!(&self.f, "{desc}\n\n")?;
        Ok(())
    }

    fn write_const(&mut self, shadow_const: ShadowConst, val: ConstVal) -> SdResult<()> {
        let desc = format!("#[doc=r#\"{}\"#]", val.desc);

        let define = match val.t {
            ConstType::OptStr => format!(
                "#[allow(dead_code)]\n\
            pub const {} :{} = r#\"{}\"#;",
                shadow_const.to_ascii_uppercase(),
                ConstType::Str.to_string(),
                ""
            ),
            ConstType::Str => format!(
                "#[allow(dead_code)]\n\
            pub const {} :{} = r#\"{}\"#;",
                shadow_const.to_ascii_uppercase(),
                ConstType::Str.to_string(),
                val.v
            ),
            ConstType::Bool => format!(
                "#[allow(dead_code)]\n\
            pub const {} :{} = {};",
                shadow_const.to_ascii_uppercase(),
                ConstType::Bool.to_string(),
                val.v.parse::<bool>().unwrap()
            ),
        };

        writeln!(&self.f, "{desc}")?;
        writeln!(&self.f, "{define}\n")?;
        Ok(())
    }

    fn gen_version(&mut self) -> SdResult<Vec<&'static str>> {
        let (ver_fn, clap_ver_fn, clap_long_ver_fn) = match self.map.get(TAG) {
            None => (
                version_branch_const(),
                clap_version_branch_const(),
                clap_long_version_branch_const(),
            ),
            Some(tag) => {
                if !tag.v.is_empty() {
                    (
                        version_tag_const(),
                        clap_version_tag_const(),
                        clap_long_version_tag_const(),
                    )
                } else {
                    (
                        version_branch_const(),
                        clap_version_branch_const(),
                        clap_long_version_branch_const(),
                    )
                }
            }
        };
        writeln!(&self.f, "{ver_fn}\n")?;
        writeln!(&self.f, "{clap_ver_fn}\n")?;
        writeln!(&self.f, "{clap_long_ver_fn}\n")?;

        Ok(vec![BUILD_CONST_VERSION, BUILD_CONST_CLAP_LONG_VERSION])
    }

    fn gen_build_in(&self, gen_const: Vec<&'static str>) -> SdResult<()> {
        let mut print_val = String::from("\n");

        // append gen const
        for k in self.map.keys() {
            let tmp = format!(r#"{}println!("{k}:{{{k}}}\n");{}"#, "\t", "\n");
            print_val.push_str(tmp.as_str());
        }

        // append gen fn
        for k in gen_const {
            let tmp = format!(r#"{}println!("{k}:{{{k}}}\n");{}"#, "\t", "\n");
            print_val.push_str(tmp.as_str());
        }

        let everything_define = format!(
            "/// print build in method\n\
            #[allow(dead_code)]\n\
            pub fn print_build_in() {\
            {{print_val}}\
            }\n",
        );
        writeln!(&self.f, "{everything_define}")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_build() -> SdResult<()> {
        Shadow::build_inner("./".into(), "./".into())?;
        let shadow = fs::read_to_string("./shadow.rs")?;
        assert!(!shadow.is_empty());
        assert!(shadow.lines().count() > 0);
        println!("{shadow}");
        Ok(())
    }

    #[test]
    fn test_env() {
        for (k, v) in std::env::vars() {
            println!("K:{k},V:{v}");
        }
    }
}
