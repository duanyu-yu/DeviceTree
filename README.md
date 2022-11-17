# device-tree

The crate **device-tree** can be used for parsing Devicetree Blob (DTB).

**Device-tree** is a `#![no_std]` crate written in Rust. 

## Example
```
use device_tree::DeviceTreeBlob;

fn main() {
    let mut dtb: &[u8] = include_bytes!("<path-to-*.dtb>");

    let mut blob = DeviceTreeBlob::from_bytes(&mut dtb).unwrap();

    let tree = blob.to_tree().unwrap();

    println!("{}", tree);
}
```
