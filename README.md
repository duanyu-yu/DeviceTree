# DeviceTree

![Crates.io](https://img.shields.io/crates/v/devicetree)

The crate **devicetree** can be used for parsing Devicetree Blob (DTB), based on [Devicetree Specification](https://www.devicetree.org/specifications/).

The crate **devicetree** is a `#![no_std]` crate written in Rust. 

## Example
```Rust
use devicetree::DeviceTreeBlob;

fn main() {
    let mut dtb: &[u8] = include_bytes!("<path-to-*.dtb>");

    let tree = DeviceTree::from_bytes(&mut dtb).unwrap();

    println!("{}", tree);
}
```

## Debug
**devicetree** uses Log Messages to log info, debug, or error messages to the console. More about Log Messages can be found [here](https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/log.html#log-messages).

Set the `RUST_LOG` environment variable to print debug messages:
```shell
RUST_LOG=debug cargo run
```
## TODO
- [ ] convert property value corresponds property type
- [ ] dtb writer
- [ ] dynamic management of devicetree
