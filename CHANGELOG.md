# Changelog

This is the changelog of [Polywave](https://crates.io/crates/polywave/).
See `README.md` for more information.

## 0.5.0

* Update signatures for `discard_alpha` methods
* Add `Hsl` and `Hsla` colours
* Add `lch` module
* Move `CieLch`, `CieLcha`, `OkLch`, and `OkLcha` to `lch`
* Add `Hwb` and `Hwba` colours
* Add `to_hwb` destructor and `from_hwb` constructor to `Hsv<{f16, f32, f64, f128}>`
* Add `to_hwba` destructor and `from_hwba` constructor to `Hsva<{f16, f32, f64, f128}>`
* Remove `to_f16_lossy`, `to_f32_lossy`, `to_f64_lossy`, and `to_f128_lossy` from `Component`
* Update docs

## 0.4.0

* Add `xyz` and `lab` modules
* Add `CieXyz` and `CieXyza` colours
* Add `CieLab`, `CieLaba`, `OkLab`, and `OkLaba` colours
* Add `CieLch`, `CieLcha`, `OkLch`, and `OkLcha` colours
* Add `OpRgb` and `OpRgba` colours
* Implement `DefinedGamut` for `CieXyz`, `CieXyza`, `CieLab`, `CieLaba`, `OkLab`, `OkLaba`, `CieLch`, `CieLcha`, `OkLch`, `OkLcha`, `OpRgb`, and `OpRgba`
* Add `with_alpha` method to `CieXyz`, `CieLab`, `OkLab`, `CieLch`, `OkLch`, and `OpRgb`
* Add `discard_alpha` method to `CieXyza`, `CieLaba`, `OkLaba`, `CieLcha`, `OkLcha`, and `OpRgba`
* Add `as_rgb` method to `OpRgb`
* Add `as_rgba` method to `OpRgba`
* Update docs

## 0.3.0

* Make `std` a default feature
* Add `Rgb` colour
* Add `Hsva` and `Hsv` colours
* Add `SRgba` and `SRgb` colours
* Add `with_alpha` method to `Rgb`, `Hsv`, and `SRgb`
* Add `discard_alpha` method to `Rgba`, `Hsva`, and `SRgba`
* Rename `Css` to `Html`
* Remove `From<T>` implementations for colours where `T` is a tuple
* Add `rgb`, `www`, and `hsv` modules
* Remove `colour` module
* Unimplement `From<Rgba<u8>>` and `Into<Rgba<u8>>` for `Html`
* Implement `From<SRgba<u8>>`, `From<SRgb<u8>>`, `Into<SRgba<u8>>`, and `Into<u32>` for `Html`
* Replace `Html::from_rgba` with `from_s_rgba`
* Replace `Html::to_rgba` with `to_s_rgba`
* Add `as_rgba` method to `SRgba`
* Add `as_rgb` method to `SRgb`
* Add `DefinedGamut` trait
* Remove `Colour::to_wgpu_color_lossy`
* Remove `wgpu` trait
* Add `to_s_rgb` method to `DefinedGamut`
* Implement `DefinedGamut` for `SRgba`, `SRgb`, and `Html`
* Rename `CssFromStrError` to `HtmlFromStrError`

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
