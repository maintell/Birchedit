# ADR 0070: The grammars in a library of their own

- Status: accepted
- Date: 2026-11-11
- Builds on [ADR 0011](0011-syntax-highlighting.md) (tree-sitter and the grammar crates) and
  [ADR 0041](0041-crate-layout.md) (what a crate is for).

## Context

The executable was 101 MB on Windows after [0.10.0](../../CHANGELOG.md) dropped the three
largest grammars, and more than half of it is parser tables: the grammars of 47 languages (and
the 4 embedded in them), each a generated C table of a few hundred kilobytes to six. A session
uses two or three of them. Every copy of Birchpad carries all of them, and every update
downloads all of them again.

The grammar crates are ordinary Rust dependencies, so the tables are linked into the binary at
build time. Nothing in the design needs them there: `Language` only asks for a
`tree_sitter::Language` when a language's queries are compiled, which happens the first time a
document of that language is parsed.

## Decision

**The parsers move into `birchpad-grammars`, a `cdylib` the executable loads.**

- `crates/grammars` depends on the grammar crates and exports one C function,
  `birchpad_grammar(id)`, which answers with the grammar's own function
  (`tree_sitter_rust`) or null.
- `crates/syntax` keeps the language table (ids, names, extensions, queries), but its
  `Language.grammar` is `None` in a build with the `dynamic-grammars` feature: the grammar of a
  language is asked of the library by its id (`crate::dynamic`), once per language, the first
  time its queries are compiled.
- The application's release builds set the feature (`crates/app`'s `dynamic-grammars`); tests,
  fuzzing and a `cargo run` link the grammars, so a test needs no library built first.
- Both sides take the grammar crates from `[workspace.dependencies]`: one version of
  `tree-sitter` and of each grammar, because a grammar's tables and the `TSLanguage` structure
  must be the same on both sides of the library boundary.
- The library is looked for next to the executable, under the platform's name for a `cdylib`
  (`birchpad_grammars.dll`, `libbirchpad_grammars.so`, `libbirchpad_grammars.dylib`), and stays
  loaded: the grammar functions point into it.
- **A language may come as a pack of its own**: built with `--no-default-features --features
  <grammar>`, `birchpad-grammars` is that grammar alone, and the executable looks for
  `<id>.dll`, `<id>.so` or `<id>.dylib` in the `grammars` folder of its data folder first, then
  in a `grammars` folder beside itself, before asking the library of all grammars. A pack a
  user adds is found at the next start; one that does not hold the language is passed over,
  with a line on standard error.
- Every package carries it: the portable ZIPs, the per-user installer (Velopack packs the
  folder it is given), the MSI (`Birchpad.wxs` names the file) and the macOS bundle, where the
  universal library is `lipo`'d beside the binary in `Contents/MacOS`.

**A build without the library is a build without highlighting, said once.** If it is missing or
does not answer, the first language that needs a grammar reports it on standard error and its
documents open as plain text; nothing else fails, and no file is touched.

## Consequences

- The executable falls to about half: the code of GPUI, wasmtime, Rhai and the rest, without the
  parser tables. The library holds the tables and is mapped only as far as a language's pages
  are read.
- The install and the update are about the same size as before, in two files instead of one.
  What an update downloads is unchanged; what the process maps at start is smaller.
- A language is now named twice: in the table (`crates/syntax/src/language.rs`) and in the
  library's match. A test compares the two lists, so that a language added to one and not the
  other fails the build rather than opening as plain text.
- The library must match the executable's tree-sitter ABI. The workspace's dependency table is
  what keeps them equal; a crate that named its own version could silently build two.
- Packaging is now file-list work in four places (zip, Velopack, MSI, macOS bundle); a package
  that forgets the library still works, without highlighting.
