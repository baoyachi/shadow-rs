[shadow-rs][docsrs]:: build-time information stored in your rust project.(binary,lib,cdylib,dylib)
========================================
<p align="center">
  <img
    width="200"
    src="https://raw.githubusercontent.com/baoyachi/shadow-rs/master/shadow-rs.png"
    alt="shadow-rs build tool"
  />
</p>

[docsrs]: https://docs.rs/shadow-rs

[![Chrono GitHub Actions](https://github.com/baoyachi/shadow-rs/workflows/build/badge.svg)](https://github.com/baoyachi/shadow-rs/actions?query=workflow%3Abuild)
[![Crates.io](https://img.shields.io/crates/v/shadow-rs.svg)](https://crates.io/crates/shadow-rs)
[![Docs.rs](https://docs.rs/shadow-rs/badge.svg)](https://docs.rs/shadow-rs)
[![Download](https://img.shields.io/crates/d/shadow-rs)](https://crates.io/crates/shadow-rs)
[![DepStatus](https://deps.rs/repo/github/baoyachi/shadow-rs/status.svg)](https://deps.rs/repo/github/baoyachi/shadow-rs)
[![Gitter](https://badges.gitter.im/shadow-rs/community.svg)](https://gitter.im/shadow-rs/community?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge)

`shadow-rs` allows you to recall properties of the build process and environment at runtime, including:

* `Cargo.toml` project version
* Dependency information
* The Git commit that produced the build artifact (binary)
* What version of the rust toolchain was used in compilation
* The build variant, e.g. `debug` or `release`
* (And more)

You can use this tool to check in production exactly where a binary came from and how it was built.

# Full Examples

* Check out the [example_shadow](https://github.com/baoyachi/shadow-rs/tree/master/example_shadow) for a simple
  demonstration of how `shadow-rs` might be used to provide build-time information at run-time.
* Check out the [example_shadow_hook](https://github.com/baoyachi/shadow-rs/tree/master/example_shadow_hook) for a
  simple demonstration of how `shadow-rs` might be used to provide build-time information at run-time,and add custom
  hook.
* built in function:[examples](https://github.com/baoyachi/shadow-rs/tree/master/examples).

# Setup Guide

### 1) Modify `Cargo.toml` fields

Modify your `Cargo.toml` like so:

```TOML
[package]
build = "build.rs"

[dependencies]
shadow-rs = "0.5"

[build-dependencies]
shadow-rs = "0.5"
```

### 2) Create `build.rs` file

Now in the root of your project (same directory as `Cargo.toml`) add a file `build.rs`:
  * with add custom `const` or `fn` see:[example_shadow_hook](https://github.com/baoyachi/shadow-rs/blob/master/example_shadow_hook/build.rs)

```rust
fn main() -> shadow_rs::SdResult<()> {
    shadow_rs::new()
}
```

### 3) Integrate shadow

In your rust file (e.g. `*.rs`):

```rust
use shadow_rs::shadow;

shadow!(build);
```

**Notice that the `shadow!` macro is provided the identifier `build`. You can now use this identifier to access
build-time information.**

### 4) Done. Use shadow.

```rust
fn main() {

    //shadow-rs built in function  
    println!("{}", shadow_rs::is_debug());      // check if this is a debug build
    println!("{}", shadow_rs::branch());        // get current project git branch. e.g.'master'
    println!("{}", shadow_rs::tag());        // get current project git head tag. e.g.'v1.5.3'

    //shadow-rs built in const   
    println!("{}", build::version());           // the version (description binary detail information)
    println!("{}", build::clap_version());      // usually used by clap crates version() (description binary detail information)
    println!("{}", build::PKG_VERSION);         // current package version. e.g. '1.3.15-beta2'  
    println!("{}", build::PKG_VERSION_MAJOR);   //current package major version. e.g. '1'  
    println!("{}", build::PKG_VERSION_MINOR);   //current package minor version. e.g. '3'  
    println!("{}", build::PKG_VERSION_PATCH);   //current package minor version. e.g. '15'  
    println!("{}", build::PKG_VERSION_PRE);   //current package minor version. e.g. 'beta2'  
    println!("{}", build::BRANCH);              // the branch, e.g. 'master'
    println!("{}", build::TAG);                 // the tag, e.g. 'v1.0.0'
    println!("{}", build::SHORT_COMMIT);        // short commit hash, e.g. '8405e28e'
    println!("{}", build::COMMIT_HASH);         // full commit hash, e.g. '8405e28e64080a09525a6cf1b07c22fcaf71a5c5'
    println!("{}", build::COMMIT_DATE);         // commit date, e.g. '2020-08-16 11:52:47'
    println!("{}", build::COMMIT_DATE_2822);    // commit date, e.g. 'Thu, 24 Jun 2021 21:33:59 +0800'
    println!("{}", build::COMMIT_DATE_3339);    // commit date, e.g. '2021-06-24T21:33:59.972494+08:00'
    println!("{}", build::COMMIT_AUTHOR);       // commit author, e.g. 'baoyachi'
    println!("{}", build::COMMIT_EMAIL);        // commit email, e.g. 'example@gmail.com'

    println!("{}", build::BUILD_OS);            // the OS that built the binary, e.g. 'macos-x86_64'
    println!("{}", build::RUST_VERSION);        // rustc version e.g. 'rustc 1.45.0 (5c1f21c3b 2020-07-13)'
    println!("{}", build::RUST_CHANNEL);        // rust toolchain e.g. 'stable-x86_64-apple-darwin (default)'
    println!("{}", build::CARGO_VERSION);       // cargo version e.g. 'cargo 1.45.0 (744bd1fbb 2020-06-15)'
    println!("{}", build::CARGO_TREE);          // e.g. the output of '$ cargo tree'

    println!("{}", build::PROJECT_NAME);        // your project name, e.g. 'shadow-rs'
    println!("{}", build::BUILD_TIME);          // time when start build occurred, e.g. '2020-08-16 14:50:25'
    println!("build_time_2822:{}", build::BUILD_TIME_2822); // time when start build occurred by rfc2822, e.g. 'Thu, 24 Jun 2021 21:33:59 +0800'
    println!("build_time_3339:{}", build::BUILD_TIME_3339); // time when start build occurred by rfc3339, e.g. '2021-06-24T21:33:59.972494+08:00'
    println!("{}", build::BUILD_RUST_CHANNEL);  // e.g. 'debug'
}
```

## Clap Example

And you can also use `shadow-rs`
with [`clap`](https://github.com/baoyachi/shadow-rs/blob/master/example_shadow/src/main.rs).

## Support const,function in table

#### shadow-rs built in function.

* how to use 👉 : [examples](https://github.com/baoyachi/shadow-rs/tree/master/examples)

| function | desc |
| ------ | ------ |
| is_debug() | check if this is a debug build.e.g.'true/false' |
| branch() | get current project branch.e.g.'master/develop' |
| tag() | get current project tag.e.g.'v1.3.5' |

#### shadow-rs support build const,function.

* how to use 👉 : [shadow_example](https://github.com/baoyachi/shadow-rs/tree/master/example_shadow)

| const/fn | example |
| ------ | ------ |
| version() | support mini version information.It's use easy. |
| clap_version() | support mini version information for clap.It's use easy. |
| BRANCH | master/develop |
| TAG | v1.0.0 |
| SHORT_COMMIT | 8405e28e |  
| COMMIT_HASH | 8405e28e64080a09525a6cf1b07c22fcaf71a5c5 |  
| COMMIT_DATE | 2020-08-16 13:12:52 |
| COMMIT_DATE_2822 | Thu, 24 Jun 2021 21:33:59 +0800 |  
| COMMIT_DATE_3339 | 2021-06-24T21:33:59.972494+08:00 |
| COMMIT_AUTHOR | baoyachi |
| COMMIT_EMAIL | xxx@gmail.com |  
| BUILD_OS | macos-x86_64 |  
| RUST_VERSION | rustc 1.45.0 (5c1f21c3b 2020-07-13) |  
| RUST_CHANNEL | stable-x86_64-apple-darwin (default) |  
| CARGO_VERSION | cargo 1.45.0 (744bd1fbb 2020-06-15) |  
| PKG_VERSION | 0.3.13 |
| CARGO_TREE | cargo tree |  
| PROJECT_NAME | shadow-rs |  
| BUILD_TIME | 2021-06-24 21:33:59 |  
| BUILD_TIME_2822 | Thu, 24 Jun 2021 21:33:59 +0800 |  
| BUILD_TIME_3339 | 2021-06-24T15:53:55+08:00 |  
| BUILD_RUST_CHANNEL | debug/release |  

If you have any questions, please create an [issue](https://github.com/baoyachi/shadow-rs/issues/new) so we may improve
the documentation where it may be unclear.

## People using shadow-rs

If you are using `shadow-rs`, please tell me! Or instead, consider making a note
here: [Shadow Users Collection](https://github.com/baoyachi/shadow-rs/issues/19).

<table>
  <tr>
    <td align="center"><a href="https://github.com/nushell/nushell"><img src="https://avatars3.githubusercontent.com/u/50749515?s=200&v=4" width="100px;" alt="nushell"/><br /><sub><b>nushell</b></sub></a><br /></td>
    <td align="center"><a href="https://github.com/starship/starship"><img src="https://raw.githubusercontent.com/starship/starship/master/media/icon.png?s=200&v=4" width="100px;" alt="starship"/><br /><sub><b>starship</b></sub></a><br /></td>
    <td align="center"><a href="https://github.com/Riey/kime"><img src="https://github.com/Riey/kime/raw/develop/docs/assets/kime-roundy-default-without-text-bluegrey.png?s=200&v=4" width="100px;" alt="kime"/><br /><sub><b>kime</b></sub></a><br /></td>
    <td align="center"><a href="https://github.com/Hirevo/alexandrie"><div>alexandrie</div><br/><sub><b>alexandrie</b></sub></a><br /></td>
  </tr>
</table>