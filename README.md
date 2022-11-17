# device-tree

The crate **device-tree** can be used for parsing Devicetree Blob (DTB), based on [Devicetree Specification](https://www.devicetree.org/specifications/).

The crate **device-tree** is a `#![no_std]` crate written in Rust. 

## Example
```Rust
use device_tree::DeviceTreeBlob;

fn main() {
    let mut dtb: &[u8] = include_bytes!("<path-to-*.dtb>");

    let tree = DeviceTree::from_bytes(&mut dtb).unwrap();

    println!("{}", tree);
}
```

## Debug
**device-tree** uses Log Messages to log info, debug, or error messages to the console. More about Log Messages can be found [here](https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/log.html#log-messages).

Set the `RUST_LOG` environment variable to print debug messages:
```shell
RUST_LOG=debug cargo run
```
