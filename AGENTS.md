# Working in this repository

Instructions for coding agents. `CONTRIBUTING.md` is the source of truth for people; this file
compresses it into rules an agent can follow without reading the whole tree first.

## What this tree is

Birchpad: a Notepad++-like text editor in Rust on GPUI (Windows, macOS, Linux; 27 crates, ~90k
lines of source, ~58k of tests). This checkout is a fork, `maintell/Birchedit`, developed from
`SecurityTrip/Birchpad`; `origin` is the fork. Updates, links and the online catalog point at
this repository (ADR 0068).

- Workspace version and MSRV live in `Cargo.toml` (`0.8.0`, Rust 1.98, edition 2024);
  `rust-toolchain.toml` pins the toolchain and rustup installs it.
- `docs/ROADMAP.md` is the plan and the record of what each release delivered;
  `docs/adr/` holds the architecture decisions (68 of them) and `CHANGELOG.md` the notes.

## Build, check, test

```bash
cargo check --workspace --all-targets     # must stay warning-free
cargo fmt --all
cargo clippy --workspace --all-targets
cargo nextest run --workspace             # or: cargo test --workspace (slower)
cargo run -p birchpad                     # the application
```

- **Do not turn on incremental compilation.** `.cargo/config.toml` keeps it off: the machines
  this fork is built on are short of disk, and the incremental directories of this workspace
  reach gigabytes. `CARGO_INCREMENTAL=1` overrides it for one command when there is room;
  otherwise a cold `cargo check --workspace --all-targets` takes about four minutes.
- CI treats warnings as errors (`CARGO_BUILD_WARNINGS=deny`); a `warning:` line is a failure.
- `cargo-nextest` is not installed by default (`cargo install cargo-nextest --locked`); the test
  settings are in `.config/nextest.toml`.
- End-to-end tests drive a real window: `cargo test -p birchpad --features e2e --test e2e`.
- Timing tests are `#[ignore]` and run in release:
  `cargo test --release -p birchpad-core -p birchpad-io -p birchpad-view -p birchpad-editor -- --ignored --nocapture --test-threads=1`.
- Fuzzing: `cargo test -p birchpad-fuzz` over `fuzz/seeds`.
- Syntax highlight snapshots: after reviewing a change,
  `BIRCHPAD_BLESS=1 cargo nextest run -p birchpad-syntax --test highlights`.

## Where code goes

- **`birchpad-core` never depends on UI crates.** Everything in it is testable without a window.
- Logic that needs no window goes to the crate of its kind (`core`, `io`, `config`, `commands`,
  `tools`, `view`, `syntax`, `git`, `lsp`, `terminal`, `update`, `plugin`), never into `app`.
  What the window's parts share (the command registry, application state, dialog buttons,
  messages to the user) is `birchpad-ui`. See ADR 0041 and ADR 0058.
- A command handler is registered for the view that runs it (`CommandTarget`), not in the
  registry. The editor asks the application through `EditorHooks`.
- **Positions in the text model are byte offsets** into the rope. Convert to chars, UTF-16 or
  visual columns only at the edges (rendering, LSP, status bar).
- A new dependency is the last resort: check `cargo tree --duplicates`, prefer what GPUI already
  builds, turn off unneeded default features. `resvg`, `similar` and `x11rb` are held at GPUI's
  versions (`.github/dependabot.yml`): do not bump them alone.
- No `unsafe` without a comment saying why it is sound.
- No `todo!()` or `unimplemented!()` land in `src`; the workspace lints warn on them.

## Tests

- **No tests in `src`.** A crate's tests live in its `tests/` folder and use its public
  interface (ADR 0043). Most are in one binary, `tests/integration`, whose `main.rs` names a
  module per part of `src` (`src/pane.rs` → `tests/integration/pane/tests.rs`).
- What only tests need goes behind the crate's `test-support` feature.
- Every change covers the positive case, the negative case (invalid input, a refused action, an
  error path) and the boundaries (empty text, first and last line, no final line break, CRLF,
  wide and combining characters, very long lines, each limit and one past it).
- A bug fix comes with a test that fails without the fix.
- Test fixtures under `tests/fixtures` are committed byte for byte (`.gitattributes`): never let
  a tool change their encoding or line endings.
- Coverage floors (`.github/coverage-floors.toml`) and mutation floors
  (`.github/mutants-floors.toml`) only rise. Deleting a feature means deleting its code and its
  tests together, and saying so in the changelog; do not lower a floor to make a build pass.

## Documentation

- **Documents for users and contributors exist twice**, English and Russian, side by side
  (`README.md` / `README.ru.md`, `CONTRIBUTING`, `SECURITY`, `PRIVACY`, `CODE_OF_CONDUCT`,
  `docs/plugins.md`, `docs/release-checklist.md`, `docs/ROADMAP.md`): a change to one changes
  the other in the same commit, and each links the other at the top. ADRs and `CHANGELOG.md`
  are English only.
- Point text rather than code: every user-visible string is a key in `crates/i18n/locales/`
  (`en.yaml`, `de.yaml`, `ru.yaml`, `zh.yaml`), used as `t!("key")`. A key a translation lacks falls back
  to English, so a new string needs at least `en.yaml`; add `de` and `ru` in the same change.
- Behaviour that contradicts an existing ADR needs a new ADR, not a silent divergence
  (`docs/adr/0068-…` is the newest).
- `CHANGELOG.md`: add to `## [Unreleased]`, under `Added` / `Changed` / `Fixed` / `Removed`.
- Per-crate versions rise only when a crate changed in a release;
  `cargo run -p birchpad-release-tool -- crate-versions` checks it.
- After a change, check that the roadmap and the changelog do not now claim something false
  (an unchecked box that shipped, a stale branch name, a table missing a crate).

## Clean room (do not skip)

Birchpad reimplements Notepad++ without its code. So:

- **Do not read the source** of Notepad++, Scintilla, Lexilla, a Notepad++ plugin (ComparePlus,
  JSON Viewer, XML Tools, Hex Editor, DSpellCheck…) or another GPL/MPL editor for the feature at
  hand. Work from the user manual, the program's behaviour, and the files it reads and writes.
- File formats (element names, keys of `session.xml`, `stylers.xml`, UDL, API files, the command
  line) may be followed so that users' files carry over; their data may not be bundled.
- Code written with an AI assistant is held to the same rules: ask for it from the description,
  never by pointing at the GPL source.

## This fork's own rules

- **Never push to `SecurityTrip/Birchpad`.** `origin` is `maintell/Birchedit`. To take upstream
  changes, `git remote add upstream https://github.com/SecurityTrip/Birchpad` and merge.
- Update checks read `birchpad-updates.json` from this repository's `updates` release
  (`crates/update/src/lib.rs`, ADR 0068); there is no update API and nothing is collected.
  Trusted keys are compiled in from `BIRCHPAD_TRUSTED_KEYS` at build time; a build without it
  trusts no manifest, which is why `updates.mode` defaults to `off`.
- The PlantUML packages (`crates/update/src/component.rs`) still come from an upstream release,
  pinned by SHA-256. A rebuilt package needs its own release and new hashes.
- The product name and package identifiers (`io.github.securitytrip.birchpad`, the MSI
  `UpgradeCode`) are unchanged: renaming them moves settings and break upgrades, so it is a
  decision of its own.

## Commits

Conventional Commits (`feat:`, `fix:`, `docs:`, `refactor:`, `test:`, `ci:`, `chore:`), one
logical change per commit, with its tests, its docs and its changelog entry. A change touching a
Notepad++ feature says in the message what it was built from (manual, behaviour, file samples).

## Known traps

- The workspace needs a recent toolchain (1.98, edition 2024); older `cargo` fails at parse time.
- GPU text, dialogs and menus can only be checked by running the application; `cargo check` does
  not catch a wrong layout or a shortcut conflict.
- Windows-only code (GDI printing, the tray, the clipboard, ConPTY) is not covered by the Linux
  and macOS jobs; test it on Windows.
- `crates/app` is the largest crate by far; find the module by feature (`file_commands.rs`,
  `find_in_files.rs`, `preferences/`, `panels/`, `workspace/`) before reading widely.
