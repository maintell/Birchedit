# Privacy policy

English · [Русский](PRIVACY.ru.md)

Birchpad collects no personal data. It has no crash reporting and no accounts, and your
documents, sessions and backups stay on your computer, in Birchpad's data folder (or, in
portable mode, the `data` folder next to it). Its update checks read a signed manifest and send
nothing about the copy that reads it; every request Birchpad makes is
[below](#network-requests), and settings turn them off.

## Network requests

Birchpad connects to the network only for updates, and to install PlantUML, plugins and
dictionaries when you ask:

- **Checking for updates**: once a day while Birchpad runs, and when you pick Help > Check for
  Updates, it downloads the update manifest of this repository's releases:
  `https://github.com/maintell/Birchedit/releases/download/updates/birchpad-updates.json`. The
  manifest is signed, and Birchpad checks it against the keys built into it, so the server
  cannot change what it says ([ADR 0068](docs/adr/0068-updates-from-this-repository.md)). With
  the `updates.url` setting (an administrator's copy of the manifest) Birchpad downloads it from
  there alone. These requests are made only when updates are turned on: `updates.mode` is
  `"off"` by default. Each of them says which version of Birchpad asks and on which operating
  system (the `User-Agent` header, such as `Birchpad/0.8.0 (windows)`), and nothing about your
  files or your copy.
- **Downloading updates**: with `updates.mode = "auto"`, a copy installed with the per-user
  Windows installer downloads the package of a newer version, from the address the manifest
  lists (GitHub's release downloads).
- **Installing PlantUML**: when you click Install PlantUML in the preview of a PlantUML
  document (or View > Preview > Install PlantUML), Birchpad downloads PlantUML and the Java that
  runs it from a release of the repository it came from, `SecurityTrip/Birchpad` (`plantuml-*`),
  once, until this repository builds its own ([ADR 0068](docs/adr/0068-updates-from-this-repository.md)).
  With `network.online-features = false` it does not.
- **The online list of plugins and dictionaries**: when you click Refresh List in Plugins
  Admin, or open Plugins > Spell Checker > Download Languages..., Birchpad reads the list from
  `https://raw.githubusercontent.com/maintell/Birchedit/main/packaging/catalog/catalog.json`
  (or the address in `network.catalog`); when you install a plugin or download a dictionary
  from it, Birchpad downloads that plugin's or dictionary's files from the addresses the list
  names, and checks them against it. With `network.online-features = false` it does neither.

These go through the proxy of Help > Set Updater Proxy... (`updates.proxy`) when one is set, else
through the one of `HTTPS_PROXY` or `ALL_PROXY` or, on Windows, the system's.

The Markdown preview loads no images from the web, and a link in it opens in your browser
only when you click it. PlantUML diagrams are drawn by PlantUML on your computer (yours, or the
one Birchpad installed), not by a server. Language servers (Preferences > Language Servers) are
programs you configure, which Birchpad starts on your computer and sends the text of your
documents of their language; Birchpad sends nothing about them anywhere else. Git
(Preferences > Git) is your own `git` program, which Birchpad asks for the last commit of your
files and the blame of a line; it never fetches, pulls or pushes.

GitHub serves the other files and, like any web server, sees the requests: the address of your
computer or network and the time. Its
[privacy statement](https://docs.github.com/site-policy/privacy-policies/github-general-privacy-statement)
covers them.

## Turning it off

- `updates.mode = "off"` (Preferences > Updates), the default, makes no requests for updates.
- `updates.mode = "notify"` checks and announces new versions, but downloads nothing.
- `network.online-features = false` stops installing PlantUML and reading the online list of
  plugins and dictionaries; with both, Birchpad makes no network requests at all.
- Administrators can set the same with the `UpdateMode` and `OnlineFeatures` policies (the
  Windows registry), which users cannot override; with `UpdateUrl`, update checks go to the
  administrator's copy of the manifest instead of GitHub's.

A link you choose to open (Open Download Page in an update notice, or Birchpad's home page,
manual and forum in the Help menu) goes to your browser.
