# AGENTS.md

Authoritative guidance for AI coding agents working in this repository.
Humans should read it too: it records decisions that were argued once and
should not be argued again, plus footguns that are not visible from
`Cargo.toml`.

---

## What this crate is

[`bq40z50-rx`](https://crates.io/crates/bq40z50-rx) is a `#![no_std]`,
async, platform-agnostic driver for the Texas Instruments BQ40Z50 battery
fuel gauge, built on `embedded-hal-async` and `embedded-batteries-async`.

**The crate is `bq40z50-rx`; the repository is `bq40z50`.** They differ.
This matters for `release-plz.toml` and for crates.io Trusted Publishing.

Four silicon revisions are supported, selected by mutually-meaningful
cargo features `r1`, `r3`, `r4`, `r5`. There is no `r2`.

---

## Required reading before changing register definitions

The four Technical Reference Manuals. The datasheet (SLUSBS8B) contains
**no register map** and defers to them, so it is almost never the right
citation.

| Feature | Lit. no. | Document |
|---|---|---|
| `r1` | SLUUA43A | [bq40z50 TRM](https://www.ti.com/lit/pdf/sluua43a) |
| `r3` | SLUUBU5A | [bq40z50-R3 TRM](https://www.ti.com/lit/pdf/sluubu5a) |
| `r4` | SLUUCH2  | [BQ40Z50-R4 TRM](https://www.ti.com/lit/pdf/sluuch2) |
| `r5` | SLUUCN4B | [BQ40Z50-R5 TRM](https://www.ti.com/lit/pdf/sluucn4b) |

Extract them with `pdftotext -layout`; without `-layout` the register
tables collapse into unusable text.

### Citation discipline

- Quote the TRM **verbatim** and cite the section, table or figure number.
- **Never cite line numbers of a text extraction.** Extractions are a
  convenience, not a citable artifact; the numbers differ per tool and
  per run.
- **Revisions genuinely differ. Never generalise a finding from one TRM
  to another.** Verify per revision, and say explicitly when a revision
  is intentionally left alone.

### No hardware

There is no BQ40Z50 on any bench attached to this project. Every claim
must be justified from the TRMs or from a test. **If a TRM is silent or
self-contradictory, stop and report it - do not guess.**

### Known TRM self-contradictions

Recorded so nobody re-derives them. None are driver bugs.

- **SLUUA43A** `ManufacturingStatus` bit 15: §12.1.29 prose says
  `CAL_EN`, §12.1.44's bit list and table say `CAL_TEST`. The bit list
  wins, so `r1` keeps `cal_test()` while r3/r4/r5 use `cal_en()`.
- **SLUUCH2 §16.1.40** defines R4 `OperationStatus` bit 17 as *both*
  `LED` and `RSVD`. The manifest exposes `led()`.
- **SLUUCN4B §16.1.38** labels R5 `SafetyStatus` bit 17 `RSVD` in the
  table but defines `HWDF` in the prose. The manifest follows the prose.
- **SLUUBU5A §15.1.68** heads a section `ManufacturerInfoB` but its body
  reads "Output 4 bytes of **ManufacturerInfo2**", a name appearing
  nowhere else, and Table 15-1 types it `Mixed`. r3's
  `MANUFACTURE_INFO_B` is therefore **deliberately left** as a `u32`
  while r4/r5 are modelled as bytes.

---

## Repository layout

```
device_r{1,3,4,5}.ddsl   register manifests, device-driver v2 DSL
src/versions/gen_r*.rs   GENERATED from the manifests - never hand-edit
src/versions/r*.rs       per-revision hand-written helpers
src/interface.rs         I2C/SMBus transport, PEC, retries, data flash
src/common.rs            embedded-batteries trait impls, Config
src/consts.rs            wire constants
src/tests.rs             one macro expanded per revision - see Gotcha 3
```

---

## Gotchas

### 1. `gen_r*.rs` is generated, and CI diffs it byte-for-byte

After **any** manifest edit:

```sh
ddc build rust -s device_rN.ddsl -o src/versions/gen_rN.rs --rust-defmt-feature=defmt-03
rustup run nightly rustfmt --edition 2024 src/versions/gen_rN.rs
```

- `ddc` is `device-driver-cli`, **pinned to 2.1.0** in
  `.github/workflows/device-driver.yml`. v1 shipped a binary called
  `device-driver-cli`; v2 renamed it to `ddc`. A version mismatch
  produces `ddc: command not found` or a byte-level diff.
- **Generate into the repository directory.** `rustfmt.toml` sets
  `max_width = 120`; generating into a temp directory silently formats at
  the default width and yields a false mismatch.
- Format with **nightly** rustfmt. `rustfmt.toml` uses unstable options.

### 2. Register changes come in pairs

Most registers appear twice: as a `command MAC_X` (ManufacturerAccess,
address `0x44xxxx`) and as a direct `register X` (SBS address). They
describe the same silicon bits and **must agree**. Fix both, in every
revision where they exist.

Direct registers carry per-field `RO` *and* a register-level
`access: RO`. MAC `fields-out` blocks carry no access modifiers on any
field - that is the established pattern, so do not add them there.

### 3. `cfg` inside the test macro is evaluated once per build

`src/tests.rs` is one `macro_rules! bq40z50_tests` expanded once per
revision module:

```rust
crate::tests::bq40z50_tests!(Bq40z50R5, 28, 26);
//                           revision,  security_keys_len, lifetime_block_1_len
```

A `#[cfg(feature = "r1")]` **inside** the macro body is evaluated once
for the whole build, not once per expansion. In a multi-revision build
every module then gets the same value and the wrong revisions fail.

**Pass revision-specific values as macro parameters instead.** This has
already been fixed once; do not reintroduce it.

Verify with `cargo test --features r1,r3,r4,r5`. CI does not catch this
class: the test matrix builds one revision at a time, and `cargo-hack`'s
powerset runs clippy, which never compiles the test target.

### 4. `src/tests.rs` is structurally odd - leave it alone

A non-`cfg(test)` module under `src/` that exists only to hold a test
macro is unusual, and the tests inside it are integration-flavoured.
This is **known** and deliberately out of scope; restructuring it is its
own pull request. Add tests in the existing style.

### 5. PEC: the two read paths are asymmetric on purpose

- `read_with_retries` **keeps** a `use_pec` parameter, because call sites
  genuinely differ: `false` for raw/variable-length reads with no PEC
  byte in the frame, `true` for the `read_mfg_info` whole-payload read.
- `mac_read_with_retries` **takes no such parameter** and reads
  `self.config.pec_read` itself. Every call site passed exactly that
  expression, so the parameter carried no information.

Do not "restore symmetry" by adding the parameter back.

### 6. Data flash invariants

- The window is `0x4000`-`0x5FFF` on all four revisions; the TRM section
  titles state it. `check_df_range` guards all three entry points and
  validates the **end** of the transfer with checked arithmetic.
- The gauge auto-increments its read pointer, so a retry after a failed
  chunk **must re-send that chunk's starting address**. Not doing so
  silently returns the next block as if it were the one requested.
- Writes commit per chunk and cannot be made atomic.
  `PartialDataFlashWrite { committed }` reports caller-payload bytes
  accepted, and is produced **only when `committed > 0`**; if the first
  chunk fails the underlying `I2c`/`Timeout` cause is returned instead,
  because there is no partial state to describe.

### 7. The `semver` job is gone on purpose

`cargo-semver-checks` runs **only** inside release-plz
(`semver_check = true`). As a CI job it fails every pull request that
makes a breaking change, because a feature branch still carries the last
published version - the bump arrives with the release PR. Restoring the
job means setting `semver_check = false` in `release-plz.toml`.

### 8. Transport is deduplicated - keep it that way

`src/interface.rs` once held two near-identical copies of all transport
logic, one per `embassy-timeout` cfg arm. They are now one copy plus a
`bus_op!` macro defined once per arm. Three separate bugs previously had
to be fixed twice. Do not reintroduce the split.

---

## The canonical local matrix

CI builds one revision at a time; several real bugs only appear in
combinations it never tries. Run all of this:

```sh
# every revision x every optional-feature combination
for rev in r1 r3 r4 r5; do
  for extra in "" "embassy-timeout," "pec-lookup-table," "embassy-timeout,pec-lookup-table,"; do
    cargo test --locked --features "$extra$rev"
  done
done

# multi-revision builds - NOT covered by CI, see Gotcha 3
cargo test --locked --features r1,r3,r4,r5

# clippy exactly as CI runs it (note: no --all-targets)
cargo clippy --locked --features <rev> -- \
  -F clippy::suspicious -F clippy::correctness -F clippy::perf -F clippy::style

cargo doc --locked --no-deps --features <rev>
cargo check --locked --target thumbv8m.main-none-eabihf --features <rev>
cargo +1.94 check --locked --features <rev>      # MSRV
cargo hack --feature-powerset clippy --locked --target thumbv8m.main-none-eabihf
cargo deny --all-features check
cargo vet check
cargo +nightly fmt --check
```

Plus the pregen check from Gotcha 1 for all four revisions.

Known noise: four `allow(clippy::all) incompatible with previous forbid`
warnings from the generated files. Pre-existing; will become a hard error
in a future compiler.

MSRV is **1.94**, set by `device-driver` 2.1.0.

---

## Verifying a fix actually works

Mock-based tests can pass against a driver that does nothing. Before
claiming a fix:

1. **Mutation-test it.** Revert the fix, confirm the test fails, and
   confirm it fails *for the right reason*. A test that still passes is
   not testing the fix - this has caught a silently-untested change here.
2. Prefer making the defect **impossible to reintroduce**. Several fixes
   here now fail to *compile* when reverted, because a type changed.
3. For refactors that must not change behaviour, diff `cargo expand`
   output before and after rather than reading the diff.

---

## Commits, pull requests and releases

### Conventional Commits

[Conventional Commits v1.0.0](https://www.conventionalcommits.org/en/v1.0.0/).
See `CONTRIBUTING.md` for the full rules.

```
<type>[optional scope][!]: <description>

[body, wrapped at 72 columns]

[footers: Closes #N, BREAKING CHANGE:, Assisted-by:]
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`,
`build`, `ci`, `chore`, `revert`.

Keep messages terse and factual. Cite TRM sections and quote them; never
cite line numbers of a text extraction.

### Squash-only: the pull request title is the commit

**This repository merges by squash only.** `allow_merge_commit` is
`false`. Individual commits on a branch are *review units*; the only
thing that lands on `main` is one squash commit built from the pull
request:

- subject = the **pull request title** (`squash_merge_commit_title: PR_TITLE`)
- body = the **pull request description** (`squash_merge_commit_message: PR_BODY`)

release-plz parses that whole commit, body included - so a
`BREAKING CHANGE:` footer or a `Closes #N` in the description is read
normally. What is lost is the branch's own commit messages: because the
body comes from `PR_BODY` rather than `COMMIT_MESSAGES`, nothing written
in an individual commit reaches `main` unless the description repeats it.

**When writing a pull request description, an agent MUST:**

1. **Aggregate severity into the title.** Scan every commit on the
   branch and take the maximum:
   - If **any** commit is breaking (`!` or a `BREAKING CHANGE:` footer),
     the title **must** carry `!` **and** the body **must** carry a
     `BREAKING CHANGE:` footer describing the combined impact. Either
     alone would bump the version correctly, but the `!` is what makes
     the break visible in `git log --oneline` and in the changelog
     heading, and the footer is what explains the migration.
   - Otherwise the title's type is the highest-ranking type present:
     `feat` > `fix` > everything else. A branch with one `feat` and nine
     `fix` commits is titled `feat`.
   - Pick a scope that covers the branch, or omit it.
2. **Collect every `Closes #N` / `Fixes #N` / `Refs #N`** from all
   commits into the pull request body. The body does reach `main`, but
   the commits it was written from do not - an issue referenced only in
   a branch commit will never close.
3. **Preserve every `BREAKING CHANGE:` footer**, merged into one footer
   in the body.
4. Keep the body terse: what changed, why, how it was verified.

Getting the title wrong means a wrong version bump or a missing release.

### AI attribution

Every commit or pull request containing AI-generated or AI-assisted work
**must** carry an `Assisted-by` trailer:

```
Assisted-by: AGENT_NAME:MODEL_VERSION [TOOL1] [TOOL2]
```

Verify your own identity rather than copying a model name from an
earlier session. Basic tooling (git, cargo, editors) is not listed.

AI agents **MUST NOT** add `Signed-off-by:`. Only a human certifies the
Developer Certificate of Origin.

### Releases are automated

Do not edit `version` in `Cargo.toml`, do not write `CHANGELOG.md`
entries, and do not push tags. `.github/workflows/release-plz.yml` opens
a release pull request; merging it publishes to crates.io via Trusted
Publishing, tags `vX.Y.Z`, and cuts the GitHub release.

Branch protection carves out `refs/heads/release-plz-*` in two rulesets
and `refs/tags/v*` in the tag-creation ruleset. Changing
`pr_branch_prefix` in `release-plz.toml` requires changing those
exclusions, or every release fails on a rule violation.

---

## Workflow expectations

1. **Verify before claiming.** Run the matrix; quote the result. "Should
   work" is not a result.
2. **Do not invent hardware behaviour.** Report gaps instead.
3. **Each commit must build clean and pass clippy and fmt on its own**,
   so `git bisect` stays useful.
4. **Treat a report as a claim.** Re-grep quotes, re-check anchors, and
   confirm a subagent's work independently before committing it.
5. **New dependencies need a `cargo-vet` entry.** `cargo vet check` is a
   CI gate.
