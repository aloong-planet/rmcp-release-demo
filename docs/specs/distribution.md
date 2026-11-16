# Distribution

## Problem Statement

The owner wants evidence that a Rust MCP server can be shipped the way future formal projects will ship
it: built for each platform from a version tag, published as a GitHub Release, and installed, upgraded
and removed by users with one command, and every release checked by actually installing it. This
repository is that trial run, so the server only has to prove that an installed copy answers MCP requests.

## Solution

`rmcp-release-demo` is a stdio MCP server with two tools, `echo` and `version`. Pushing a version tag
builds it for macOS on Apple Silicon, Linux x86_64 and Linux ARM64 and publishes a GitHub Release that
includes a curl installer; a stable release also updates the formula in the `aloong-planet/homebrew-tap`
tap. After every release, CI installs the release through each supported install method and talks MCP to
the installed binary. Pull requests merge only through a CI gate.

## User Stories

### distribution::US-001 Install on macOS with Homebrew

A user on macOS with Apple Silicon installs the server from the tap, so that an MCP client can launch it.

- distribution::US-001/AC-01: On macOS on Apple Silicon, `brew install aloong-planet/tap/rmcp-release-demo`
  installs the latest stable release, and the installed `rmcp-release-demo` completes an MCP `initialize`
  over stdio.

### distribution::US-002 Install with the curl installer

A user on Linux, or on macOS without Homebrew, installs the server with one command and gets the binary at
a fixed path to put in an MCP client's config.

- distribution::US-002/AC-01: On Linux x86_64, the README's curl command installs the latest stable release
  to `~/.local/bin/rmcp-release-demo`, and that binary completes an MCP `initialize` over stdio.
- distribution::US-002/AC-02: On Linux ARM64, the README's curl command installs the latest stable release
  to `~/.local/bin/rmcp-release-demo`, and that binary completes an MCP `initialize` over stdio.
- distribution::US-002/AC-03: On macOS on Apple Silicon, the README's curl command installs the latest
  stable release to `~/.local/bin/rmcp-release-demo`, and that binary completes an MCP `initialize` over
  stdio.
- distribution::US-002/AC-04: With `latest/download` in the command replaced by `download/v<version>`, the
  command installs release `<version>` instead of the latest stable release.
- distribution::US-002/AC-05: Running the command again replaces the installed binary with the latest stable
  release.
- distribution::US-002/AC-06: When `~/.local/bin` is not on PATH, the installer adds it for new login shells
  through a line in `~/.profile` that sources the installer's `rmcp-release-demo/env.sh` in the user's
  config directory (`$XDG_CONFIG_HOME`, by default `~/.config`).

### distribution::US-003 Use the server from an MCP client

A user points an MCP client at the installed binary as a stdio server and calls its tools.

- distribution::US-003/AC-01: `tools/list` returns two tools, `echo` and `version`.
- distribution::US-003/AC-02: Calling `echo` with a `text` argument returns a text result equal to that
  argument.
- distribution::US-003/AC-03: Calling `version` returns a text result equal to the `serverInfo.version` that
  `initialize` reports.

### distribution::US-004 Release by pushing a tag

The maintainer publishes a release by pushing a version tag, with no manual build or upload.

- distribution::US-004/AC-01: Pushing tag `v<version>`, where `<version>` is the package version, publishes
  GitHub Release `v<version>` containing an archive for each target in distribution::CON-001, a sha256
  checksum for each archive, and the curl installer script.
- distribution::US-004/AC-02: The binaries in release `v<version>` report `serverInfo.version` equal to
  `<version>`.
- distribution::US-004/AC-03: For a stable version (one without a SemVer pre-release suffix), the release
  updates the `rmcp-release-demo` formula in `aloong-planet/homebrew-tap` to that version.
- distribution::US-004/AC-04: For a version with a SemVer pre-release suffix (such as `0.1.0-rc.1`), the
  GitHub Release is marked as a prerelease and the tap is left unchanged.

### distribution::US-005 Post-release check

After every release, the maintainer sees whether the release installs and works through each supported
install method.

- distribution::US-005/AC-01: The post-release check runs automatically as the last step of every release
  run, and can be re-run by hand for a published release.
- distribution::US-005/AC-02: For a stable release, the check installs the release with Homebrew on macOS on
  Apple Silicon and sends `initialize`, `tools/list` and a `tools/call` of each tool to the installed binary,
  checking the results against distribution::US-003.
- distribution::US-005/AC-03: The check installs the release with the curl installer on macOS on Apple
  Silicon and sends the same requests, with the same checks.
- distribution::US-005/AC-04: The check installs the release with the curl installer on Linux x86_64 and
  sends the same requests, with the same checks.
- distribution::US-005/AC-05: The check installs the release with the curl installer on Linux ARM64 and
  sends the same requests, with the same checks.
- distribution::US-005/AC-06: A failed install, request or result check marks the release run failed.

## Functional Requirements

### distribution::REQ-001 The README documents the supported install methods

- distribution::REQ-001/AC-01: The README offers exactly the supported install methods: Homebrew on macOS on
  Apple Silicon, and the curl installer on macOS on Apple Silicon, Linux x86_64 and Linux ARM64. It does not
  offer Homebrew on Linux.
- distribution::REQ-001/AC-02: For each install method, the README gives the install command and the
  installed binary's absolute path for MCP client configs.
- distribution::REQ-001/AC-03: For each install method, the README gives how to upgrade:
  `brew upgrade rmcp-release-demo`, or running the curl command again.
- distribution::REQ-001/AC-04: For each install method, the README gives how to uninstall:
  `brew uninstall rmcp-release-demo`, or deleting `~/.local/bin/rmcp-release-demo` only, leaving the
  installer's `rmcp-release-demo` config directory and the lines it added to shell startup files, because
  they put `~/.local/bin` on PATH for every tool installed there.
- distribution::REQ-001/AC-05: The README shows how to install a specific version with the curl installer.
- distribution::REQ-001/AC-06: The README states, one sentence each, that Intel Mac and Windows are not
  supported.

### distribution::REQ-002 Pull requests merge only through the CI gate

- distribution::REQ-002/AC-01: A branch rule on `main` requires the `CI gate` check to pass before a pull
  request can merge.
- distribution::REQ-002/AC-02: A pull request whose changes fail `cargo lint` fails `CI gate`.
- distribution::REQ-002/AC-03: A pull request whose changes fail the tests fails `CI gate`.
- distribution::REQ-002/AC-04: A pull request whose changes fail the build for any target in
  distribution::CON-001 fails `CI gate`.

### distribution::REQ-003 MIT license (direct acceptance)

The project is licensed under MIT: the repository has an MIT `LICENSE` file, the package declares
`license = "MIT"`, and the Homebrew formula declares the MIT license.

## Constraints

### distribution::CON-001 Release targets (direct acceptance)

Applies to: every release; it builds exactly three targets: `aarch64-apple-darwin`,
`x86_64-unknown-linux-musl` and `aarch64-unknown-linux-musl`.

### distribution::CON-002 Static Linux binaries (direct acceptance)

Applies to: the Linux binaries of every release; they are statically linked and load no shared libc, so
they run whatever libc the host has.

### distribution::CON-003 stdout carries only MCP messages (direct acceptance)

Applies to: the running server; stdout carries only MCP JSON-RPC messages, and everything else, logs
included, goes to stderr.

### distribution::CON-004 Tap token scope (direct acceptance)

Applies to: the token the release pipeline uses to push to `aloong-planet/homebrew-tap`; it is a
fine-grained token with access to `aloong-planet/homebrew-tap` only, whose only permission beyond the
read-only Metadata access every fine-grained token carries is Contents read and write.

## Failure modes and boundaries

- **A target fails to build.** The release publishes nothing: no GitHub Release appears and the tap is
  unchanged, so no platform gets a partial release.
- **Pushing the formula to the tap fails for a stable release.** The GitHub Release is already published,
  because it is created before the formula is pushed, so the curl installer installs the new version; the
  tap keeps the previous formula, the post-release check does not run, and the release run is marked failed.
- **Prerelease and the post-release check.** For a prerelease the post-release check skips the Homebrew
  install and runs the three curl installs of distribution::US-005/AC-03 to AC-05, because the tap does not
  carry prereleases (distribution::US-004/AC-04).
- **A stable release fails the post-release check.** The release run is marked failed
  (distribution::US-005/AC-06) and nothing is rolled back: the tap keeps pointing at the failed version
  until the maintainer publishes a new release or reverts the tap commit by hand.
- **The post-release check for a stable release that is not the newest.** For a stable tag the check installs
  with the README's commands as they are, which install the newest stable release, and the tap carries only
  the newest formula. Run by hand (distribution::US-005/AC-01) for an older stable tag, the check therefore
  fails: the installed binary does not report the tag's version (distribution::US-004/AC-02). For a
  prerelease the curl installs name the tag, so the check verifies that prerelease even when a newer release
  exists.
- **Content the server returns.** The server renders and loads no external content; it returns text to the
  MCP client, so no renderer-created interaction surface exists on its side.

## Implementation Decisions

- The server speaks MCP over stdio through `rmcp`, the official Rust MCP SDK; the protocol version is the
  one the SDK negotiates.
- Tool contracts: `echo` takes one required string argument `text` and returns it as a single text content
  item; `version` takes no arguments and returns the package version as a single text content item.
- The package version is the only source of the release version: tags are `v` followed by the package
  version, and both `serverInfo.version` and the `version` tool report it.
- cargo-dist builds and publishes releases from GitHub Actions to GitHub Releases, with the shell and
  Homebrew installers. It publishes the formula to `aloong-planet/homebrew-tap`, does not publish
  prereleases to Homebrew, and the shell installer installs into `~/.local/bin/`.
- The post-release check belongs to the release pipeline and runs after the release is published; it also
  has a manual trigger.
- The GitHub repository `aloong-planet/rmcp-release-demo` is public, because both install methods download
  release assets anonymously.

## Out of Scope

- Intel Mac (`x86_64-apple-darwin`): the owner does not need it, and Homebrew is phasing out Intel macOS.
- Windows: the owner does not need it.
- Homebrew on Linux as a supported install method: the formula's Linux section works, but the README does
  not offer it and the post-release check does not verify it.
- Developer ID signing and notarization of the macOS binary: Homebrew and curl installs carry no quarantine
  attribute, so Gatekeeper does not block them; an archive downloaded through a browser is blocked.

## Further Notes

- This repository is a test project for the distribution flow. Formal projects will reuse the same tap,
  `aloong-planet/homebrew-tap`.
