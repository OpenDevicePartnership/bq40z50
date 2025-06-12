# BQ40Z50 Rust Device Driver

A `#[no_std]` platform-agnostic driver for the [BQ40Z50](https://www.ti.com/lit/ug/sluua43a/sluua43a.pdf) family of lithium-ion battery fuel/gas gauge, capable of managing a 1- to 4-cell battery, using the [embedded-hal](https://docs.rs/embedded-hal) traits.

A higher level API exposing standard Smart Battery Specification functions is built on top of the lower level register accessor using the [embedded-batteries](https://github.com/OpenDevicePartnership/embedded-batteries) traits.


## MSRV

Currently, rust `1.85` and up is supported.

## License

Licensed under the terms of the [MIT license](http://opensource.org/licenses/MIT).

## Contribution

Unless you explicitly state otherwise, any contribution submitted for
inclusion in the work by you shall be licensed under the terms of the
MIT license.

License: MIT
