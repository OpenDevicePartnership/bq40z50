[![no-std](https://github.com/OpenDevicePartnership/bq40z50/actions/workflows/nostd.yml/badge.svg)](https://github.com/OpenDevicePartnership/bq40z50/actions/workflows/nostd.yml)
[![check](https://github.com/OpenDevicePartnership/bq40z50/actions/workflows/check.yml/badge.svg)](https://github.com/OpenDevicePartnership/bq40z50/actions/workflows/check.yml)
[![Documentation](https://docs.rs/bq40z50-rx/badge.svg)](https://docs.rs/bq40z50-rx)
[![Crates.io Version](https://img.shields.io/crates/v/bq40z50-rx)](https://crates.io/crates/bq40z50-rx)
[![LICENSE](https://img.shields.io/badge/License-MIT-blue)](./LICENSE)

# BQ40Z50 Rust Device Driver

A `#[no_std]` platform-agnostic driver for the [BQ40Z50](https://www.ti.com/product/BQ40Z50) family of lithium-ion battery fuel/gas gauge, capable of managing a 1- to 4-cell battery, using the [embedded-hal](https://docs.rs/embedded-hal) traits.

A higher level API exposing standard Smart Battery Specification functions is built on top of the lower level register accessor using the [embedded-batteries](https://github.com/OpenDevicePartnership/embedded-batteries) traits.

Four revisions of the chip are currently supported:
- [BQ40Z50](https://www.ti.com/lit/ug/sluua43a/sluua43a.pdf)
- [BQ40Z50-R3](https://www.ti.com/lit/ug/sluubu5a/sluubu5a.pdf)
- [BQ40Z50-R4](https://www.ti.com/lit/ug/sluuch2/sluuch2.pdf)
- [BQ40Z50-R5](https://www.ti.com/lit/ug/sluucn4b/sluucn4b.pdf)

Please choose which revision you are using by enabling the appropriate feature.

## MSRV

Currently, rust `1.85` and up is supported.

## License

Licensed under the terms of the [MIT license](http://opensource.org/licenses/MIT).

## Contribution

Unless you explicitly state otherwise, any contribution submitted for
inclusion in the work by you shall be licensed under the terms of the
MIT license.

License: MIT
