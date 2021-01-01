[shadow-rs][docsrs]: build-time information stored in your binary
========================================
![shadow](./shadow-rs.png)

[![Chrono GitHub Actions][gh-image]][gh-checks]
[![Chrono on crates.io][cratesio-image]][cratesio]
[![Chrono on docs.rs][docsrs-image]][docsrs]
[![dependency on depstatus][depstatus-image]][depstatus]

[gh-image]: https://github.com/baoyachi/shadow-rs/workflows/build/badge.svg
[gh-checks]: https://github.com/baoyachi/shadow-rs/actions?query=workflow%3Abuild
[cratesio-image]: https://img.shields.io/crates/v/shadow-rs.svg
[cratesio]: https://crates.io/crates/shadow-rs
[docsrs-image]: https://docs.rs/shadow-rs/badge.svg
[docsrs]: https://docs.rs/shadow-rs
[depstatus-image]: https://deps.rs/repo/github/baoyachi/shadow-rs/status.svg
[depstatus]:https://deps.rs/repo/github/baoyachi/shadow-rs


`shadow-rs` allows you to recall properties of the build process and environment at runtime, including:

* `Cargo.toml` project version
* Dependency information
* The Git commit that produced the build artifact (binary)
* What version of the rust toolchain was used in compilation
* The build variant, e.g. `debug` or `release`
* (And more)

You can use this tool to check in production exactly where a binary came from and how it was built.

# Full Examples
* Check out the [shadow_example](https://github.com/baoyachi/shadow-rs/tree/master/example_shadow) for a simple demonstration of how `shadow-rs` might be used to provide build-time information at run-time.

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

```rust
fn main() -> shadow_rs::SdResult<()> {
    shadow_rs::new()
}
```

### 3) Integrate Shadow

In your root rust file (e.g. `main.rs`, or `lib.rs`):

```rust
#[macro_use]
extern crate shadow_rs;

shadow!(build);
```

**Notice that the `shadow!` macro is provided the identifier `build`.  You can now use this identifier to access build-time information.**

### 4) Done. Use Shadow.

```rust
fn main() {
  println!("{}", shadow_rs::is_debug());      // check if this is a debug build
  
  println!("{}", build::version());           // the version (TODO: clarify difference between ::version() and ::PKG_VERSION)
  println!("{}", build::BRANCH);              // the branch, e.g. 'master'
  println!("{}", build::SHORT_COMMIT);        // short commit hash, e.g. '8405e28e'
  println!("{}", build::COMMIT_HASH);         // full commit hash, e.g. '8405e28e64080a09525a6cf1b07c22fcaf71a5c5'
  println!("{}", build::COMMIT_DATE);         // commit date, e.g. '2020-08-16T06:22:24+00:00'
  println!("{}", build::COMMIT_AUTHOR);       // commit author, e.g. 'baoyachi'
  println!("{}", build::COMMIT_EMAIL);        // commit email, e.g. 'example@gmail.com'
  
  println!("{}", build::BUILD_OS);            // the OS that built the binary, e.g. 'macos-x86_64'
  println!("{}", build::RUST_VERSION);        // rustc version e.g. 'rustc 1.45.0 (5c1f21c3b 2020-07-13)'
  println!("{}", build::RUST_CHANNEL);        // rust toolchain e.g. 'stable-x86_64-apple-darwin (default)'
  println!("{}", build::CARGO_VERSION);       // cargo version e.g. 'cargo 1.45.0 (744bd1fbb 2020-06-15)'
  println!("{}", build::PKG_VERSION);         // e.g. '0.3.13'
  println!("{}", build::CARGO_TREE);          // e.g. the output of '$ cargo tree'
  
  println!("{}", build::PROJECT_NAME);        // your project name, e.g. 'shadow-rs'
  println!("{}", build::BUILD_TIME);          // time when build occurred (TODO: start? or end?), e.g. '2020-08-16 14:50:25'
  println!("{}", build::BUILD_RUST_CHANNEL);  // e.g. 'debug'
}
```

## Clap Example 
And you can also use `shadow-rs` with [`clap`](https://github.com/baoyachi/shadow-rs/blob/master/example_shadow/src/main.rs).

## Support const table
| const | example |
| ------ | ------ |
| version() | master/develop |
| BRANCH | master/develop |
| SHORT_COMMIT | 8405e28e |  
| COMMIT_HASH | 8405e28e64080a09525a6cf1b07c22fcaf71a5c5 |  
| COMMIT_DATE | 2020-08-16T06:22:24+00:00 |
| COMMIT_AUTHOR | baoyachi |
| COMMIT_EMAIL | xxx@gmail.com |  
| BUILD_OS | macos-x86_64 |  
| RUST_VERSION | rustc 1.45.0 (5c1f21c3b 2020-07-13) |  
| RUST_CHANNEL | stable-x86_64-apple-darwin (default) |  
| CARGO_VERSION | cargo 1.45.0 (744bd1fbb 2020-06-15) |  
| PKG_VERSION | 0.3.13 |
| CARGO_TREE | cargo tree |  
| PROJECT_NAME | shadow-rs |  
| BUILD_TIME | 2020-08-16 14:50:25 |  
| BUILD_RUST_CHANNEL | debug/release |  

If you have any questions, please create an [issue](https://github.com/baoyachi/shadow-rs/issues/new) so we may improve the documentation where it may be unclear.

## People using Shadow
If you are using `shadow-rs`, please tell me! Or instead, consider making a note here: [Shadow Users Collection](https://github.com/baoyachi/shadow-rs/issues/19).

<table>
  <tr>
    <td align="center"><a href="https://github.com/nushell/nushell"><img src="https://avatars3.githubusercontent.com/u/50749515?s=200&v=4" width="100px;" alt="nushell"/><br /><sub><b>nushell</b></sub></a><br /></td>
  </tr>
</table>

# License

## MIT License

```
Copyright (c) 2020 baoyachi

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```
