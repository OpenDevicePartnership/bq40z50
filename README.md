[![no-std](https://github.com/OpenDevicePartnership/bq40z50/actions/workflows/nostd.yml/badge.svg)](https://github.com/OpenDevicePartnership/bq40z50/actions/workflows/nostd.yml)
[![check](https://github.com/OpenDevicePartnership/bq40z50/actions/workflows/check.yml/badge.svg)](https://github.com/OpenDevicePartnership/bq40z50/actions/workflows/check.yml)
[![test](https://github.com/OpenDevicePartnership/bq40z50/actions/workflows/test.yml/badge.svg)](https://github.com/OpenDevicePartnership/bq40z50/actions/workflows/test.yml)
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

Currently, rust `1.94` and up is supported.

## Testing

The test workflow runs each chip revision with and without `embassy-timeout` and
`pec-lookup-table`. To run tests locally, select a revision and any optional features:

```sh
cargo test --locked --features r1
cargo test --locked --features r1,embassy-timeout,pec-lookup-table
```

## Data flash writes

`write_dataflash` commits data in chunks of up to 32 bytes and is **not atomic**.
If a write fails, earlier chunks are not rolled back, and the failing chunk may
also have reached the gauge. Errors do not report how many bytes were committed.
Retrying the entire write rewrites earlier chunks; callers must verify the
affected data and handle recovery.

## License

Licensed under the terms of the [MIT license](http://opensource.org/licenses/MIT).

## Contribution

Unless you explicitly state otherwise, any contribution submitted for
inclusion in the work by you shall be licensed under the terms of the
MIT license.

See [CONTRIBUTING.md](https://github.com/OpenDevicePartnership/bq40z50/blob/main/CONTRIBUTING.md)
for the commit-message convention (Conventional Commits) and the pull
request rules. AI coding agents should read
[AGENTS.md](https://github.com/OpenDevicePartnership/bq40z50/blob/main/AGENTS.md).

License: MIT
