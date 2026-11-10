# ADR 0068: Updates from this repository, and no install statistics

- Status: accepted
- Date: 2026-11-10
- Supersedes the Update API and the install statistics of
  [ADR 0064](0064-update-api-and-install-statistics.md), and the addresses in
  [ADR 0020](0020-signed-updates-and-nightly-builds.md) that named another repository.

## Context

This tree is a fork of Birchpad: `maintell/Birchedit`, developed from
`SecurityTrip/Birchpad`. It keeps Birchpad's design, its signed update manifest (ADR 0020) and
its releases; it does not keep the service behind them.

Two things in the tree belonged to the project it came from, not to this one:

- the Update API, `https://api.birchpad.ru/v1/updates/check`, a service of Birchpad's author
  that answered with the signed manifest and kept a row per copy that asked (ADR 0064);
- addresses of `SecurityTrip/Birchpad`: the manifest, the online catalog, the help menu's
  links, and the defaults of the scripts that make a signing key.

A fork that kept them would send its users' update checks to a service nobody here runs, and
would tell them about releases of another project. It would also gain nothing: the statistics
exist to decide what the author supports, and this fork has no such data to collect.

## Decision

### The manifest comes from this repository, and from nowhere else by default

`DEFAULT_MANIFEST` is the `birchpad-updates.json` asset of the `updates` release of
`maintell/Birchedit`. `updates.url` still names a mirror, read instead when it is set: the
signature, not the server, is what makes a manifest trusted, as before.

### No Update API, no install statistics

The `POST` to an update service, the install id in `state.toml`, `updates.statistics`, the
`UpdateStatistics` policy and the `statistics` module are gone, with their tests. A check
downloads the manifest and sends nothing: no id, no version, no platform. An installation that
ran an earlier build and wrote an install id keeps the line in `state.toml`; nothing reads it,
and it goes at the next save of the state.

A service of this fork's own would be a new ADR, and it would have to justify collecting
anything at all.

### Updates are off until they are turned on

`updates.mode` defaults to `off`. Two reasons, either sufficient: this repository has no
releases yet, so a check has nothing to find; and release builds compile in the trusted keys
of `BIRCHPAD_TRUSTED_KEYS`, which this repository has not set, so a build that checked would
refuse every manifest it read (`crates/update/build.rs`) and report an error to its user.

When the fork publishes releases signed with its own key, an ADR may raise the default to
`notify`; nothing else has to change.

### What still names the project this came from

- **The PlantUML bundles** (`plantuml-1.2026.9-3`): 60 MB packages of PlantUML, its EPL build
  with ELK, and a `jlink` Java runtime, built by `.github/workflows/plantuml.yml`. They are
  served from that release of `SecurityTrip/Birchpad` until this repository builds its own, and
  the SHA-256 of each package is compiled in (`crates/update/src/component.rs`): a rebuilt
  package is a new release and new hashes, not a new URL alone.
- **The product name**, the crate names and the package identifiers
  (`io.github.securitytrip.birchpad`, the MSI's `UpgradeCode`). Renaming them changes where
  settings live and how installers upgrade, so it is a decision of its own.

## Consequences

- A check works as soon as this repository publishes an `updates` release and sets
  `BIRCHPAD_TRUSTED_KEYS` and `BIRCHPAD_UPDATE_KEY` (`packaging/updates/README.md`); until
  then, nothing tries.
- The privacy policy says what is now true: no statistics, no update service, and the manifest
  downloaded from GitHub like any other release asset.
- `crates/update` loses `uuid`, its only use.
- ADR 0064 stays as the record of what Birchpad did; its API and statistics are superseded
  here.
