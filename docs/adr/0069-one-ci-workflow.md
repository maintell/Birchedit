# ADR 0069: One CI workflow, one run per event

- Status: accepted
- Date: 2026-11-10
- Replaces the workflow files that [ADR 0009](0009-releases-portable-mode-and-update-check.md),
  [ADR 0020](0020-signed-updates-and-nightly-builds.md),
  [ADR 0045](0045-plantuml-installed-on-request.md) and
  [ADR 0068](0068-updates-from-this-repository.md) named, with the same jobs.

## Context

The tree came with eleven workflow files: `linux.yml`, `windows.yml`, `macos.yml` and
`checks.yml`, all four started by every push to `main` and every pull request; `release.yml` and
`nightly.yml` calling `build.yml` as a reusable workflow; `update-manifest.yml` called by
`nightly.yml` and started by a published release; and `performance.yml`, `mutants.yml` and
`plantuml.yml` on their own triggers.

That is four runs for one push, four places to read one change's result, and eleven files to keep
in step. On a fork nobody watches `main`: its pushes started four runs that nobody asked for, and
the schedules fired three runs an hour apart for work that does not depend on each other.

GitHub Actions starts one run per workflow file per event, so a run per event and one place to
look are the same decision.

## Decision

**One workflow file, `.github/workflows/ci.yml`,** holding every job of the eleven, with a
`prepare` job in front that says what the run is: `checks`, `release`, `nightly`, `build`,
`manifest`, `plantuml` or nothing. Every other job runs on `needs.prepare.outputs.mode` and
carries the name it had, so that the jobs are still recognisable (`CI / test (ubuntu-latest)`).

- **A pull request** runs the checks and the tests: formatting, cargo-deny, coverage, fuzzing,
  Clippy and the tests on Ubuntu (with Debian trixie and bookworm), Windows x64 and ARM64, and
  macOS, and the end-to-end tests of each.
- **A tag `v*`** builds the release and leaves it as a draft; **a tag `plantuml-*`** builds the
  PlantUML packages.
- **A published release** signs the update manifest, as before.
- **The night at 00:00 UTC** does the nightly build, the timing checks and mutation testing in
  one run, side by side.
- **By hand**, `workflow_dispatch` chooses any of them: `checks`, `build` (artifacts only, no
  release), `release`, `nightly`, `manifest`, `plantuml`.
- **A push to `main`** starts nothing: a pull request's run is the gate, and a release is a tag.

Reusable workflows (`workflow_call`) cannot be called from the workflow that declares them, so
`build.yml` and `update-manifest.yml` became jobs of `ci.yml`: `keys`, `build`, `sign-executables`,
`package-windows`, `sign-installers`, `sbom`, `publish`/`publish-nightly` and `manifest`.

Publishing is stricter than it was: a release is created only when every job before it succeeded
and its artifacts exist (`needs.package-windows.result == 'success'`), not merely when nothing
failed, so a skipped Windows package can no longer produce a release without Windows files.

## Consequences

- One run per event, and a run's jobs say which of them a given event skipped: a pull request
  shows the release jobs as skipped, which is honest and quickly read.
- Status checks are named after the workflow (`CI / test (ubuntu-latest)`). A branch ruleset
  written for `Linux / test (ubuntu-latest)` has to be re-pointed once; the fork's has none.
- The nightly build, the timing checks and mutation testing share a run: if one is red the run is
  red, though each job's result is its own. Mutation testing no longer has a run of its own to
  retry separately; it is a job like any other.
- The checks jobs keep the same commands and the same order of work; only the file that declares
  them changed.
