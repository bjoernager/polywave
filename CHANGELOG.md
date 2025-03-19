# Changelog

This is the changelog of [Polywave](https://crates.io/crates/polywave/).
See `README.md` for more information.

## 0.2.0

* Add `Colour` trait
* Unimplement `From<Rgba<u8>>` for `wgpu::Color`
* Add `to_wgpu_color_lossy` method to `Colour`
* Add `Css` colour
* Implement `Colour` for `Rgba` and `Css`
* Rename `RgbaU8FromStrError` to `CssFromStrError`
* Add `from_u32` constructor and `to_u32` destructor to `Css`
* Add CSS named colours as associated constants to `Css`
* Implement `From<u32>` and `Into<u32>` for `Css`
* Implement `From<Rgba<u8>>` and `Into<Rgba<u8>>` for `Css`
* Update tests
* Add `from_rgba` constructor and `to_rgba` destructor to `Css`
* Add `colour` module
* Remove `from_u32` and `to_u32` from `Rgba`
* Unimplement `Display` for `Rgba`
* Implement `FromStr` for `Css`
* Unimplement `FromStr` for `Rgba`
* Fix wgpu conversions not transferring colour channels
* Add `std` feature

## 0.1.0

* Replace `Srgba8` with generic `Rgba`
* Add `Component` trait
* Add `f16` and `f128` features
* Add `to_u32` conversion destructor to `Rgba<u8>`
* Add `from_u32` constructor to `Rgba<u8>`
* Replace `zerocopy::FromBytes` with `zerocopy::FromZeros` for `Rgba`
* Replace `bytemuck::AnyBitPattern` with `bytemuck::Zeroable` for `Rgba`
* Do not implement `bytemuck::NoUninit` for `Rgba`
* Implement `Display` for `Rgba<u8>`
* Implement `Component` for `u8`, `i8`, `u16`, `i16`, `u32`, `i32`, `u64`, `i64`, `u128`, `i128`, `f16`, `f32`, `f64`, and `f128`
* Implement `FromStr` for `Rgba<u8>`
* Add `error` module
* Add `RgbaU8FromStrError` error type
* Add tests
* Rename project to *Polywave* (from *Sibgha*)
* Rename `sibgha` crate to `polywave`
* Add docs
* Update lints
* Add readme
* Add `to_f16_lossy`, `to_f32_lossy`, `to_f64_lossy`, and `to_f128_lossy` methods to `Component`

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
