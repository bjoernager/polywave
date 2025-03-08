# Changelog

This is the changelog of [Sibgha](https://crates.io/crates/sibgha/).
See `README.md` for more information.

## 0.0.0

* Configure lints
* License under MPL 2.0
* Add changelog
* Add Cargo manifest
* Add gitignore
* Add `Srgba8` type
* Add `zerocopy` and `bytemuck` features
* Add `serde` feature
* Add `wgpu` feature
* Add `new` and `from_u32` constructors to `Srgba8`
* Add `get` and `into_u32` destructors to `Srgba8`
* Implement `Debug` and `Display` for `Srgba8`
* Implement `Default` for `Srgba8`
* Implement `From<(u8, u8, u8, u8)>` and `From<u32>` for `Srgba8`
* Implement `Clone` and `Copy` for `Srgba8`
* Implement `PartialEq`, `Eq`, `PartialOrd`, and `Ord` for `Srgba8`
* Implement `zerocopy::FromBytes`, `zerocopy::Immutable`, and `zerocopy::IntoBytes` for `zerocopy::Srgba8`
* Implement `bytemuck::AnyBitPattern` and `bytemuck::NoUninit` for `Srgba8`
* Implement `serde::Deserialize` and `serde::Serialize` for `Srgba8`
* Implement `From<Srgba8>` for `wgpu::Color`
* Implement `Hash` for `Srgba8`
* Enable `no_std`
