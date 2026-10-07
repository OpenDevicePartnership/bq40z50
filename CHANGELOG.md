# Changelog

All notable changes to `bq40z50-rx` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Breaking

- Correct direct/MAC status mappings
  ([OpenDevicePartnership/bq40z50#93][pr-93]):
  - R3/R4/R5: move `emshut()` to bit 6, expose bit 29 as `disconn()`,
    rename `opncell()` to `force()`, and remove `PfAlert::opnc()`.
  - Replace `slpad()` with `vlb()` on R4 and `storagem()` on R5.
  - Remove `SafetyStatus::ptos()` on all revisions; retain
    `SafetyAlert::ptos()`.

### Added

- Add missing protection/shutdown flags on R3/R4/R5, R5 temperature/watchdog
  and TMP468 status, R4/R5 OCV prediction, and R5 low-battery gauging status
  ([OpenDevicePartnership/bq40z50#93][pr-93]).

[pr-93]: https://github.com/OpenDevicePartnership/bq40z50/pull/93
