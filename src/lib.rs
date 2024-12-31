#![doc(html_logo_url = "https://raw.githubusercontent.com/baoyachi/shadow-rs/master/shadow-rs.png")]
//! `shadow-rs`: Build-time information stored in your Rust project (binary, lib, cdylib, dylib).
//!
//! `shadow-rs` allows you to access properties of the build process and environment at runtime, including:
//!
//! * `Cargo.toml` information, such as the project version
//! * Dependency information
//! * Git information, such as the commit that produced the build artifact
//! * What version of the Rust toolchain was used in compilation
//! * The build variant, e.g. `debug` or `release`
//! * ... And more!
//!
//! You can use this crate to programmatically check where a binary came from and how it was built.
//!
//! # Examples
//! * Check out the [example_shadow](https://github.com/baoyachi/shadow-rs/tree/master/example_shadow) for a simple demonstration of how `shadow-rs` might be used to provide build-time information at run-time.
//! * Check out the [example_shadow_hook](https://github.com/baoyachi/shadow-rs/tree/master/example_shadow_hook) for a demonstration of how custom hooks can be used to add extra information to `shadow-rs`'s output.
//! * Check out the [`builtin_fn` example](https://github.com/baoyachi/shadow-rs/tree/master/examples/builtin_fn.rs) for a simple demonstration of the built-in functions that `shadow-rs` provides.
//!
//! # Setup
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
//! fn main() {
//!     ShadowBuilder::builder()
//!         .build_pattern(BuildPattern::RealTime)
//!         .build().unwrap();
//! }
//! ```
//!
//! If you want to exclude some build constants, you can use [`new_deny`] instead of [`new`].
//!
//! ### 3) Integrate Shadow
//! In your main Rust file (usually `main.rs` or `lib.rs`), add this:
//!
//! ```ignore
//! use shadow_rs::shadow;
//!
//! shadow!(build);
//! ```
//!
//! The `shadow!` macro uses the given identifier to create a module with that name.
//!
//! ### 4) Use Shadow Constants
//! You can now use the module defined with `shadow!` to access build-time information.
//!
//! ```ignore
//! fn main(){
//!     println!("debug:{}", shadow_rs::is_debug()); // check if this is a debug build. e.g 'true/false'
//!     println!("branch:{}", shadow_rs::branch()); // get current project branch. e.g 'master/develop'
//!     println!("tag:{}", shadow_rs::tag()); // get current project tag. e.g 'v1.3.5'
//!     println!("git_clean:{}", shadow_rs::git_clean()); // get current project clean. e.g 'true/false'
//!     println!("git_status_file:{}", shadow_rs::git_status_file()); // get current project statue file. e.g '  * examples/builtin_fn.rs (dirty)'
//!
//!     println!("{}", build::VERSION); //print version const
//!     println!("{}", build::CLAP_LONG_VERSION); //print CLAP_LONG_VERSION const
//!     println!("{}", build::BRANCH); //master
//!     println!("{}", build::SHORT_COMMIT);//8405e28e
//!     println!("{}", build::COMMIT_HASH);//8405e28e64080a09525a6cf1b07c22fcaf71a5c5
//!     println!("{}", build::COMMIT_DATE);//2021-08-04 12:34:03 +00:00
//!     println!("{}", build::COMMIT_AUTHOR);//baoyachi
//!     println!("{}", build::COMMIT_EMAIL);//xxx@gmail.com
//!
//!     println!("{}", build::BUILD_OS);//macos-x86_64
//!     println!("{}", build::RUST_VERSION);//rustc 1.45.0 (5c1f21c3b 2020-07-13)
//!     println!("{}", build::RUST_CHANNEL);//stable-x86_64-apple-darwin (default)
//!     println!("{}", build::CARGO_VERSION);//cargo 1.45.0 (744bd1fbb 2020-06-15)
//!     println!("{}", build::PKG_VERSION);//0.3.13
//!     println!("{}", build::CARGO_TREE); //like command:cargo tree
//!     println!("{}", build::CARGO_MANIFEST_DIR); // /User/baoyachi/shadow-rs/ |
//!
//!     println!("{}", build::PROJECT_NAME);//shadow-rs
//!     println!("{}", build::BUILD_TIME);//2020-08-16 14:50:25
//!     println!("{}", build::BUILD_RUST_CHANNEL);//debug
//!     println!("{}", build::GIT_CLEAN);//false
//!     println!("{}", build::GIT_STATUS_FILE);//* src/lib.rs (dirty)
//! }
//! ```
//!
//! ## Clap
//! You can also use `shadow-rs` to provide information to command-line interface crates such as [`clap`](https://docs.rs/clap/latest/clap/). An example of this can be found in [`example_shadow`](https://github.com/baoyachi/shadow-rs/blob/master/example_shadow/src/main.rs).
//!
//! For the user guide and further documentation, see the [README of `shadow-rs`](https://github.com/baoyachi/shadow-rs).
//!
//! # List of Generated Output Constants
//!
//! All constants produced by `shadow-rs` are documented in the module created with [`shadow!`], so `rustdoc` and your IDE will pick it up.
//!
//! ```
//! pub const PKG_VERSION: &str = "1.3.8-beta3";
//! pub const PKG_VERSION_MAJOR: &str = "1";
//! pub const PKG_VERSION_MINOR: &str = "3";
//! pub const PKG_VERSION_PATCH: &str = "8";
//! pub const PKG_VERSION_PRE: &str = "beta3";
//! pub const RUST_VERSION: &str = "rustc 1.45.0 (5c1f21c3b 2020-07-13)";
//! pub const BUILD_RUST_CHANNEL: &str = "debug";
//! pub const COMMIT_AUTHOR: &str = "baoyachi";
//! pub const BUILD_TIME: &str = "2020-08-16 13:48:52";
//! pub const BUILD_TIME_2822: &str = "Thu, 24 Jun 2021 21:44:14 +0800";
//! pub const BUILD_TIME_3339: &str = "2021-06-24T15:53:55+08:00";
//! pub const COMMIT_DATE: &str = "2021-08-04 12:34:03 +00:00";
//! pub const COMMIT_DATE_2822: &str = "Thu, 24 Jun 2021 21:44:14 +0800";
//! pub const COMMIT_DATE_3339: &str = "2021-06-24T21:44:14.473058+08:00";
//! pub const COMMIT_EMAIL: &str = "xxx@gmail.com";
//! pub const PROJECT_NAME: &str = "shadow-rs";
//! pub const RUST_CHANNEL: &str = "stable-x86_64-apple-darwin (default)";
//! pub const BRANCH: &str = "master";
//! pub const CARGO_LOCK: &str = r#"
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
//! pub const CARGO_VERSION: &str = "cargo 1.45.0 (744bd1fbb 2020-06-15)";
//! pub const BUILD_OS: &str = "macos-x86_64";
//! pub const COMMIT_HASH: &str = "386741540d73c194a3028b96b92fdeb53ca2788a";
//! pub const GIT_CLEAN: bool = true;
//! pub const GIT_STATUS_FILE: &str = "* src/lib.rs (dirty)";
//! ```
//!
#[cfg(feature = "metadata")]
pub extern crate cargo_metadata;
#[cfg(feature = "metadata")]
pub extern crate serde_json;

mod build;
mod ci;
mod date_time;
mod env;
mod err;
mod gen_const;
mod git;
mod hook;

/// Re-exported from the is_debug crate
pub use is_debug::*;

use build::*;

use crate::ci::CiType;
pub use crate::date_time::DateTime;
pub use const_format::*;
use std::collections::{BTreeMap, BTreeSet};
use std::env as std_env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::gen_const::{
    cargo_metadata_fn, clap_long_version_branch_const, clap_long_version_tag_const,
    clap_version_branch_const, clap_version_tag_const, version_branch_const, version_tag_const,
    BUILD_CONST_CLAP_LONG_VERSION, BUILD_CONST_VERSION,
};
pub use err::{SdResult, ShadowError};

pub use crate::build::{BuildPattern, ShadowBuilder};
use crate::hook::HookExt;
pub use {build::ShadowConst, env::*, git::*};

pub trait Format {
    fn human_format(&self) -> String;
}

const DEFINE_SHADOW_RS: &str = "shadow.rs";

pub const CARGO_CLIPPY_ALLOW_ALL: &str =
    "#[allow(clippy::all, clippy::pedantic, clippy::restriction, clippy::nursery)]";

/// Add a module with the provided name which contains the build information generated by `shadow-rs`.
///
/// # Example
///
/// ```ignore
/// use shadow_rs::shadow;
///
/// shadow!(my_build_information);
///
/// fn main() {
///     println!("I'm version {}!", my_build_information::VERSION);
/// }
/// ```
///
/// The convention, however, is to use `shadow!(build);`.
#[macro_export]
macro_rules! shadow {
    ($build_mod:ident) => {
        #[doc = r#"shadow-rs mod"#]
        pub mod $build_mod {
            include!(concat!(env!("OUT_DIR"), "/shadow.rs"));
        }
    };
}

/// Generates build information for the current project.
/// This function must be called from `build.rs`.
///
/// # Example
///
/// ```ignore
/// // build.rs
///
/// fn main() -> shadow_rs::SdResult<()> {
///    shadow_rs::new()
/// }
/// ```
#[deprecated(
    since = "0.37.0",
    note = "Please use  [`ShadowBuilder::builder`] instead"
)]
pub fn new() -> SdResult<()> {
    ShadowBuilder::builder().build()?;
    Ok(())
}

/// Since [cargo metadata](https://crates.io/crates/cargo_metadata) details about workspace
/// membership and resolved dependencies for the current package, storing this data can result in
/// significantly larger crate sizes. As such, the CARGO_METADATA const is disabled by default.
///
/// Should you choose to retain this information, you have the option to customize a deny_const
/// object and override the `new_deny` method parameters accordingly.
///
#[allow(clippy::all, clippy::pedantic, clippy::restriction, clippy::nursery)]
pub fn default_deny() -> BTreeSet<ShadowConst> {
    BTreeSet::from([CARGO_METADATA])
}

/// Identical to [`new`], but additionally accepts a build output denylist.
/// This list determines constants to be excluded in the build output.
///
/// Note that not all constants can be excluded independently, since some constants depend on others.
/// See [`ShadowConst`] for a list of constants that can be excluded.
///
/// # Example
///
/// ```ignore
/// // build.rs
///
/// use std::collections::BTreeSet;
///
/// fn main() -> shadow_rs::SdResult<()> {
///    let mut deny = BTreeSet::new();
///    deny.insert(shadow_rs::CARGO_TREE);
///    shadow_rs::new_deny(deny)
/// }
/// ```
#[deprecated(
    since = "0.37.0",
    note = "Please use  [`ShadowBuilder::builder`] instead"
)]
pub fn new_deny(deny_const: BTreeSet<ShadowConst>) -> SdResult<()> {
    ShadowBuilder::builder().deny_const(deny_const).build()?;
    Ok(())
}

/// Identical to [`new`], but additionally accepts an output hook.
///
/// The hook receives the output file of `shadow-rs`, and it can add additional entries to the
/// output of `shadow-rs` by writing to this file.
/// Note that since the output will be compiled as a Rust module, inserting invalid Rust code will lead to a compile error later on.
///
/// # Example
///
/// ```ignore
/// // build.rs
///
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
///
#[deprecated(
    since = "0.37.0",
    note = "Please use  [`ShadowBuilder::builder`] instead"
)]
pub fn new_hook<F>(f: F) -> SdResult<()>
where
    F: HookExt,
{
    ShadowBuilder::builder()
        .deny_const(f.default_deny())
        .hook(f.hook_inner())
        .build()?;
    Ok(())
}

/// Returns the contents of [`std::env::vars`] as an ordered map.
pub(crate) fn get_std_env() -> BTreeMap<String, String> {
    let mut env_map = BTreeMap::new();
    for (k, v) in std_env::vars() {
        env_map.insert(k, v);
    }
    env_map
}

/// `shadow-rs` configuration.
///
/// This struct encapsulates the configuration for the `shadow-rs` build process. It allows for fine-grained control over
/// various aspects of the build, including file output, build constants, environment variables, deny lists, and build patterns.
///
/// While it is possible to construct a [`Shadow`] instance manually, it is highly recommended to use the [`ShadowBuilder`] builder pattern structure
/// provided by `shadow-rs`. The builder pattern simplifies the setup process and ensures that all necessary configurations are properly set up,
/// allowing you to customize multiple aspects simultaneously, such as using a denylist and a hook function at the same time.
///
/// # Fields
///
/// * `f`: The file that `shadow-rs` writes build information to. This file will contain serialized build constants and other metadata.
/// * `map`: A map of build constant identifiers to their corresponding `ConstVal`. These are the values that will be written into the file.
/// * `std_env`: A map of environment variables obtained through [`std::env::vars`]. These variables can influence the build process.
/// * `deny_const`: A set of build constant identifiers that should be excluded from the build process. This can be populated via [`ShadowBuilder::deny_const`].
/// * `out_path`: The path where the generated files will be placed. This is usually derived from the `OUT_DIR` environment variable but can be customized via [`ShadowBuilder::out_path`].
/// * `build_pattern`: Determines the strategy for triggering package rebuilds (`Lazy`, `RealTime`, or `Custom`). This affects when Cargo will rerun the build script and can be configured via [`ShadowBuilder::build_pattern`].
///
/// # Example
///
/// ```no_run
/// use std::collections::BTreeSet;
/// use shadow_rs::{ShadowBuilder, BuildPattern, CARGO_TREE, CARGO_METADATA};
///
/// ShadowBuilder::builder()
///    .build_pattern(BuildPattern::RealTime)
///    .deny_const(BTreeSet::from([CARGO_TREE, CARGO_METADATA]))
///    .build().unwrap();
/// ```
///
#[derive(Debug)]
pub struct Shadow {
    /// The file that `shadow-rs` writes build information to.
    ///
    /// This file will contain all the necessary information about the build, including serialized build constants and other metadata.
    pub f: File,

    /// The values of build constants to be written.
    ///
    /// This is a mapping from `ShadowConst` identifiers to their corresponding `ConstVal` objects. Each entry in this map represents a build constant that will be included in the final build.
    pub map: BTreeMap<ShadowConst, ConstVal>,

    /// Build environment variables, obtained through [`std::env::vars`].
    ///
    /// These environment variables can affect the build process and are captured here for consistency and reproducibility.
    pub std_env: BTreeMap<String, String>,

    /// Constants in the deny list, passed through [`ShadowBuilder::deny_const`].
    ///
    /// This set contains build constant identifiers that should be excluded from the build process. By specifying these, you can prevent certain constants from being written into the build file.
    pub deny_const: BTreeSet<ShadowConst>,

    /// The output path where generated files will be placed.
    ///
    /// This specifies the directory where the build script will write its output. It's typically set using the `OUT_DIR` environment variable but can be customized using [`ShadowBuilder::out_path`].
    pub out_path: String,

    /// Determines the strategy for triggering package rebuilds.
    ///
    /// This field sets the pattern for how often the package should be rebuilt. Options include `Lazy`, `RealTime`, and `Custom`, each with its own implications on the build frequency and conditions under which a rebuild is triggered.
    /// It can be configured using [`ShadowBuilder::build_pattern`].
    pub build_pattern: BuildPattern,
}

impl Shadow {
    /// Write the build configuration specified by this [`Shadow`] instance.
    /// The hook function is run as well, allowing it to append to `shadow-rs`'s output.
    pub fn hook<F>(&self, f: F) -> SdResult<()>
    where
        F: Fn(&File) -> SdResult<()>,
    {
        let desc = r#"/// Below code generated by project custom from by build.rs"#;
        writeln!(&self.f, "\n{desc}\n")?;
        f(&self.f)?;
        Ok(())
    }

    /// Try to infer the CI system that we're currently running under.
    ///
    /// TODO: Recognize other CI types, especially Travis and Jenkins.
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

        CiType::None
    }

    /// Checks if the specified build constant is in the deny list.
    ///
    /// # Arguments
    /// * `deny_const` - A value of type `ShadowConst` representing the build constant to check.
    ///
    /// # Returns
    /// * `true` if the build constant is present in the deny list; otherwise, `false`.
    pub fn deny_contains(&self, deny_const: ShadowConst) -> bool {
        self.deny_const.contains(&deny_const)
    }

    /// Create a new [`Shadow`] configuration with a provided denylist.
    /// The project source path and output file are automatically derived from Cargo build environment variables.
    #[deprecated(
        since = "0.37.0",
        note = "Please use  [`ShadowBuilder::builder`] instead"
    )]
    pub fn build(deny_const: BTreeSet<ShadowConst>) -> SdResult<Shadow> {
        ShadowBuilder::builder().deny_const(deny_const).build()
    }

    #[deprecated(
        since = "0.37.0",
        note = "Please use  [`ShadowBuilder::builder`] instead"
    )]
    pub fn build_with(
        src_path: String,
        out_path: String,
        deny_const: BTreeSet<ShadowConst>,
    ) -> SdResult<Shadow> {
        ShadowBuilder::builder()
            .deny_const(deny_const)
            .src_path(src_path)
            .out_path(out_path)
            .build()
    }

    fn build_inner(builder: ShadowBuilder) -> SdResult<Shadow> {
        let out_path = builder.get_out_path()?;
        let src_path = builder.get_src_path()?;
        let build_pattern = builder.get_build_pattern().clone();
        let deny_const = builder.get_deny_const().clone();

        let out = {
            let path = Path::new(out_path);
            if !out_path.ends_with('/') {
                path.join(format!("{out_path}/{DEFINE_SHADOW_RS}"))
            } else {
                path.join(DEFINE_SHADOW_RS)
            }
        };

        let mut shadow = Shadow {
            f: File::create(out)?,
            map: Default::default(),
            std_env: Default::default(),
            deny_const,
            out_path: out_path.to_string(),
            build_pattern,
        };
        shadow.std_env = get_std_env();

        let ci_type = shadow.try_ci();
        let src_path = Path::new(src_path.as_str());

        let mut map = new_git(src_path, ci_type, &shadow.std_env);
        for (k, v) in new_project(&shadow.std_env) {
            map.insert(k, v);
        }
        for (k, v) in new_system_env(&shadow) {
            map.insert(k, v);
        }
        shadow.map = map;

        // deny const
        shadow.filter_deny();

        shadow.write_all()?;

        // handle hook
        if let Some(h) = builder.get_hook() {
            shadow.hook(h.hook_inner())?
        }

        Ok(shadow)
    }

    fn filter_deny(&mut self) {
        self.deny_const.iter().for_each(|x| {
            self.map.remove(&**x);
        })
    }

    fn write_all(&mut self) -> SdResult<()> {
        self.gen_header()?;

        self.gen_const()?;

        //write version function
        let gen_version = self.gen_version()?;

        self.gen_build_in(gen_version)?;

        Ok(())
    }

    /// Request Cargo to re-run the build script if any environment variable observed by this [`Shadow`] configuration changes.
    #[deprecated(
        since = "0.37.0",
        note = "Please use  [`ShadowBuilder::build_pattern`] instead"
    )]
    pub fn cargo_rerun_if_env_changed(&self) {
        for k in self.std_env.keys() {
            println!("cargo:rerun-if-env-changed={k}");
        }
    }

    /// Request Cargo to re-run the build script if any of the specified environment variables change.
    /// This function is not influenced by this [`Shadow`] configuration.
    #[deprecated(
        since = "0.37.0",
        note = "Please use  [`ShadowBuilder::build_pattern`] instead"
    )]
    pub fn cargo_rerun_env_inject(&self, env: &[&str]) {
        for k in env {
            println!("cargo:rerun-if-env-changed={}", *k);
        }
    }

    fn gen_const(&mut self) -> SdResult<()> {
        let out_dir = &self.out_path;
        self.build_pattern.rerun_if(self.map.keys(), out_dir);

        for (k, v) in self.map.clone() {
            self.write_const(k, v)?;
        }
        Ok(())
    }

    fn gen_header(&self) -> SdResult<()> {
        let desc = format!(
            r#"// Code automatically generated by `shadow-rs` (https://github.com/baoyachi/shadow-rs), do not edit.
// Author: https://www.github.com/baoyachi
// Generation time: {}
"#,
            DateTime::now().to_rfc2822()
        );
        writeln!(&self.f, "{desc}\n\n")?;
        Ok(())
    }

    fn write_const(&mut self, shadow_const: ShadowConst, val: ConstVal) -> SdResult<()> {
        let desc = format!("#[doc=r#\"{}\"#]", val.desc);
        let define = match val.t {
            ConstType::Str => format!(
                "#[allow(dead_code)]\n\
                {}\n\
            pub const {} :{} = r#\"{}\"#;",
                CARGO_CLIPPY_ALLOW_ALL,
                shadow_const.to_ascii_uppercase(),
                ConstType::Str,
                val.v
            ),
            ConstType::Bool => format!(
                "#[allow(dead_code)]\n\
            	{}\n\
            pub const {} :{} = {};",
                CARGO_CLIPPY_ALLOW_ALL,
                shadow_const.to_ascii_uppercase(),
                ConstType::Bool,
                val.v.parse::<bool>().unwrap()
            ),
            ConstType::Slice => format!(
                "#[allow(dead_code)]\n\
            	{}\n\
            pub const {} :{} = &{:?};",
                CARGO_CLIPPY_ALLOW_ALL,
                shadow_const.to_ascii_uppercase(),
                ConstType::Slice,
                val.v.as_bytes()
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
        for (k, v) in &self.map {
            let tmp = match v.t {
                ConstType::Str | ConstType::Bool => {
                    format!(r#"{}println!("{k}:{{{k}}}\n");{}"#, "\t", "\n")
                }
                ConstType::Slice => {
                    format!(r#"{}println!("{k}:{{:?}}\n",{});{}"#, "\t", k, "\n",)
                }
            };
            print_val.push_str(tmp.as_str());
        }

        // append gen fn
        for k in gen_const {
            let tmp = format!(r#"{}println!("{k}:{{{k}}}\n");{}"#, "\t", "\n");
            print_val.push_str(tmp.as_str());
        }

        let everything_define = format!(
            "/// Prints all built-in `shadow-rs` build constants to standard output.\n\
            #[allow(dead_code)]\n\
            {CARGO_CLIPPY_ALLOW_ALL}\n\
            pub fn print_build_in() {\
            {{print_val}}\
            }\n",
        );
        writeln!(&self.f, "{everything_define}")?;

        writeln!(&self.f, "{}", cargo_metadata_fn(self))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_build() -> SdResult<()> {
        ShadowBuilder::builder()
            .src_path("./")
            .out_path("./")
            .build()?;
        let shadow = fs::read_to_string(DEFINE_SHADOW_RS)?;
        assert!(!shadow.is_empty());
        assert!(shadow.lines().count() > 0);
        Ok(())
    }

    #[test]
    fn test_build_deny() -> SdResult<()> {
        ShadowBuilder::builder()
            .src_path("./")
            .out_path("./")
            .deny_const(BTreeSet::from([CARGO_TREE]))
            .build()?;

        let shadow = fs::read_to_string(DEFINE_SHADOW_RS)?;
        assert!(!shadow.is_empty());
        assert!(shadow.lines().count() > 0);
        // println!("{shadow}");
        let expect = "pub const CARGO_TREE :&str";
        assert!(!shadow.contains(expect));
        Ok(())
    }

    #[test]
    fn test_env() {
        for (k, v) in std::env::vars() {
            println!("K:{k},V:{v}");
        }
    }
}
