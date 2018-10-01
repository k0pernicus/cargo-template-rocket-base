# cargo-template-rocket-base

A base cargo-template for building a Rocket application

### Warning

To use the Rocket crate, you must use a Nighly version of Rust.

```bash
rustup install nightly
# OR, if you already have a nightly version
rustup update nightly
```

*Do not forget to override the default toolchain*

### How to use it ?

1. Install [cargo-generate](https://github.com/ashleygwilliams/cargo-generate):
```bash
cargo install cargo-generate
```
2. Clone the Rocket application:
```bash
cargo generate --git https://github.com/k0pernicus/cargo-template-rocket-base --name yourprojectname
```
3. Override the default toolchain in `yourprojectname`:
```bash
cd yourprojectname && rustup override set nightly
```
4. Run the app:
```bash
cargo run
```
