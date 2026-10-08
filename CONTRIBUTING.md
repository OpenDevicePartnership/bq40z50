# Contributing to Open Device Partnership

The Open Device Partnership project welcomes your suggestions and contributions! Before opening your first issue or pull request, please review our
[Code of Conduct](CODE_OF_CONDUCT.md) to understand how our community interacts in an inclusive and respectful manner.

## Contribution Licensing

Most of our code is distributed under the terms of the [MIT license](LICENSE), and when you contribute code that you wrote to our repositories,
you agree that you are contributing under those same terms. In addition, by submitting your contributions you are indicating that
you have the right to submit those contributions under those terms.

## Other Contribution Information

If you wish to contribute code or documentation authored by others, or using the terms of any other license, please indicate that clearly in your
pull request so that the project team can discuss the situation with you.

## Commit Message

Use [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/)
v1.0.0 format. The subject line follows:

```
<type>[optional scope]: <description>
```

**Allowed types** (lowercase, no trailing punctuation in the subject):

- `feat` - a new feature
- `fix` - a bug fix
- `docs` - documentation only
- `style` - formatting, whitespace (no semantic change)
- `refactor` - code change that neither fixes a bug nor adds a feature
- `perf` - a performance improvement
- `test` - adding or fixing tests
- `build` - build system or external dependencies
- `ci` - CI configuration and scripts
- `chore` - maintenance work that doesn't fit elsewhere
- `revert` - reverts a previous commit

**Subject line rules:**

- Imperative mood ("add", not "adds" or "added")
- 50 characters or less when feasible, hard limit 72
- No trailing period

**Body** (optional, encouraged for non-trivial changes):

- Separate subject from body with a blank line, wrap at 72 characters
- Explain *what* and *why*, not *how*
- Reference issues by number (`Closes #42`, `Refs #17`)
- When citing a Technical Reference Manual, quote it verbatim and cite
  the section, table or figure number. Never cite line numbers of a text
  extraction of the PDF.

**Breaking changes:** append `!` after the type/scope and include a
`BREAKING CHANGE:` footer describing the impact and migration path:

```
fix(common)!: type ChargingVoltageOverride fields as i16

SLUUCN4B Table 16-1 types 0x00B0 "Signed Int", mV. The five fields were
u16, so a negative override decoded as a large positive value.

Closes #83

BREAKING CHANGE: ChargingVoltageOverride fields are now i16.
```

**AI attribution** - see [AGENTS.md](AGENTS.md). Every commit produced
with AI assistance must end with an `Assisted-by:` trailer.

## PR Etiquette

* Create a draft PR first
* Make sure that your branch has `.github` folder and all the code linting/sanity check workflows are passing in your draft PR before sending it out to code reviewers.

## The Pull Request Is The Commit Message

**This repository merges by squash only.** The commits on your branch are
review units; the only thing that lands on `main` is a single squash
commit whose subject is the **pull request title** and whose body is the
**pull request description**.

Releases are automated and parse that commit, body included, so the
title must itself be a valid Conventional Commit and the description
carries the footers. In particular:

* If **any** commit on the branch is breaking, the title must carry `!`
  and the description must carry a `BREAKING CHANGE:` footer covering
  the combined impact.
* Otherwise use the highest-ranking type on the branch: `feat` outranks
  `fix`, which outranks everything else.
* **List every `Closes #N` in the description.** The description reaches
  `main`, but the commits it was written from do not - an issue
  referenced only in a branch commit will never close.

A wrong title means a wrong version bump or a missing release.

## Clean Commit History

Even though merging squashes them, please keep the branch history clean -
it is what reviewers read:

* Each commit builds successfully without warning
* Each commit passes clippy and `cargo fmt --check` on its own, so
  `git bisect` stays useful
* Miscellaneous commits to fix typos + formatting are squashed

## Releases

Releases are fully automated by [release-plz](https://release-plz.dev).
Do not edit `version` in `Cargo.toml`, do not write `CHANGELOG.md`
entries, and do not push tags.

On every push to `main`, release-plz opens or updates a release pull
request that bumps the version and regenerates the changelog from the
conventional commit subjects since the last tag. Merging that pull
request publishes to crates.io, pushes the `vX.Y.Z` tag, and cuts the
GitHub release.

## Regressions

When reporting a regression, please ensure that you use `git bisect` to find the first offending commit, as that will help us finding the culprit a lot faster.
