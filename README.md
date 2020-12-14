[shadow-rs][docsrs]: A build script write by Rust 
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


A build script tool compile project much information,version info,dependence info.Like shadow,if compiled,never change.forever follow your project.

When you published a rust binary. Sometimes you need to know the dependencies of the current project.
* Git version information
* which version of rust compiled. 
* Is it a debug or release version of rust.
* Cargo dependent `cargo.lock` detail crates info.
* ...

We can use this information to quickly trace the history version of the project.
n
This project can do this things that can help you quick get above information. let's configuration in your project/.

## example
You can also see [shadow_example](https://github.com/baoyachi/shadow-rs/tree/master/example_shadow) how to configuration.

## step 1
In your `cargo.toml` `packgae` with package add with below config 
```toml
[package]
build = "build.rs"

[dependencies]
shadow-rs = "0.4"

[build-dependencies]
shadow-rs = "0.4"
```

## step 2
In your project add file `build.rs`,then add with below config 
```rust
fn main() -> shadow_rs::SdResult<()> {
    shadow_rs::Shadow::new()
}
```

## step 3
In your project find `bin` rust file.It's usually `main.rs`, you can find `[bin]` file with `Cargo.toml`,then add with below config
The `shadow!(build)` with `build` config,add Rust build mod in your project. Also you also use other name that you want configuration.

```rust
#[macro_use]
extern crate shadow_rs;

shadow!(build);
```

## step 4
Then you can use const that's shadow build it(main.rs).
The `build` mod just we use `shadow!(build)` generated. 

```rust
fn main() {
    println!("{}",build::BRANCH); //master
    println!("{}",build::SHORT_COMMIT);//8405e28e
    println!("{}",build::COMMIT_HASH);//8405e28e64080a09525a6cf1b07c22fcaf71a5c5
    println!("{}",build::COMMIT_DATE);//2020-08-16T06:22:24+00:00
    println!("{}",build::COMMIT_AUTHOR);//baoyachi
    println!("{}",build::COMMIT_EMAIL);//xxx@gmail.com

    println!("{}",build::BUILD_OS);//macos-x86_64
    println!("{}",build::RUST_VERSION);//rustc 1.45.0 (5c1f21c3b 2020-07-13)
    println!("{}",build::RUST_CHANNEL);//stable-x86_64-apple-darwin (default)
    println!("{}",build::CARGO_VERSION);//cargo 1.45.0 (744bd1fbb 2020-06-15)
    println!("{}",build::PKG_VERSION);//0.3.13
    println!("{}",build::CARGO_TREE); //like command:cargo tree

    println!("{}",build::PROJECT_NAME);//shadow-rs
    println!("{}",build::BUILD_TIME);//2020-08-16 14:50:25
    println!("{}",build::BUILD_RUST_CHANNEL);//debug
}
```

## Clap example 
And you can also use const with [clap](https://github.com/baoyachi/shadow-rs/blob/master/example_shadow/src/main.rs#L24_L26).

## Support const table
| const | example |
| ------ | ------ |
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

If you have any question,you can create [issue](https://github.com/baoyachi/shadow-rs/issues/new).

## who use shadow-rs
If you are using `shadow-rs`,please tell me.Or you can make a notes here,if you like:[collection use](https://github.com/baoyachi/shadow-rs/issues/19).

<table>
  <tr>
    <td align="center"><a href="https://github.com/nushell/nushell"><img src="https://avatars3.githubusercontent.com/u/50749515?s=200&v=4" width="100px;" alt="nushell"/><br /><sub><b>nushell</b></sub></a><br /></td>
  </tr>
</table>
