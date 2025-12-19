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
//! shadow-rs = { version = "{latest version}", default-features = false }
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
//! pub const BUILD_TIMESTAMP: i64 = 1624548839;
//! pub const COMMIT_DATE: &str = "2021-08-04 12:34:03 +00:00";
//! pub const COMMIT_DATE_2822: &str = "Thu, 24 Jun 2021 21:44:14 +0800";
//! pub const COMMIT_DATE_3339: &str = "2021-06-24T21:44:14.473058+08:00";
//! pub const COMMIT_TIMESTAMP: i64 = 1624548854;
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

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "metadata")]
pub extern crate cargo_metadata;
#[cfg(feature = "metadata")]
pub extern crate serde_json;

#[cfg(feature = "build")]
mod build;
#[cfg(feature = "build")]
mod ci;
#[cfg(feature = "build")]
mod date_time;
#[cfg(feature = "build")]
mod env;
#[cfg(feature = "build")]
mod err;
#[cfg(feature = "build")]
mod gen_const;
#[cfg(feature = "build")]
mod git;
#[cfg(feature = "build")]
mod hook;
#[cfg(feature = "build")]
mod shadow;

/// Re-exported from the const_format crate
pub use const_format::*;
/// Re-exported from the is_debug crate
pub use is_debug::*;

#[cfg(feature = "build")]
mod pub_export {
    pub use crate::build::{BuildPattern, ShadowBuilder};
    pub use crate::date_time::DateTime;
    pub use crate::err::{SdResult, ShadowError};
    pub use crate::shadow::Shadow;
    pub use {crate::build::default_deny, crate::build::ShadowConst, crate::env::*, crate::git::*};

    pub trait Format {
        fn human_format(&self) -> String;
    }
}

#[cfg(feature = "build")]
pub use pub_export::*;

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
