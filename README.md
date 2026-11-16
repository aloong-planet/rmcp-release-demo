# rmcp-release-demo

A stdio [MCP](https://modelcontextprotocol.io) server written in Rust with
[rmcp](https://github.com/modelcontextprotocol/rust-sdk). It has two tools:

- `echo` returns the `text` argument unchanged.
- `version` returns the server's version.

It exists to try out a release flow: version tags build it for each platform and publish a GitHub Release
(stable versions also update a Homebrew formula), and every release is installed and checked by CI.

## Install

| Platform | Install method | Installed binary |
|---|---|---|
| macOS on Apple Silicon | Homebrew | `/opt/homebrew/bin/rmcp-release-demo` |
| macOS on Apple Silicon | curl installer | `~/.local/bin/rmcp-release-demo` |
| Linux x86_64 | curl installer | `~/.local/bin/rmcp-release-demo` |
| Linux ARM64 | curl installer | `~/.local/bin/rmcp-release-demo` |

Intel Macs are not supported. Windows is not supported.

### Homebrew (macOS on Apple Silicon)

```sh
brew install aloong-planet/tap/rmcp-release-demo
```

The binary is at `/opt/homebrew/bin/rmcp-release-demo` (`$(brew --prefix)/bin/rmcp-release-demo` if your
Homebrew lives elsewhere).

- Upgrade: `brew upgrade rmcp-release-demo`
- Uninstall: `brew uninstall rmcp-release-demo`

### curl installer (macOS on Apple Silicon, Linux x86_64, Linux ARM64)

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/aloong-planet/rmcp-release-demo/releases/latest/download/rmcp-release-demo-installer.sh | sh
```

This installs the latest stable release to `~/.local/bin/rmcp-release-demo`, that is
`/Users/<you>/.local/bin/rmcp-release-demo` on macOS and `/home/<you>/.local/bin/rmcp-release-demo` on Linux.
When `~/.local/bin` is not on your `PATH`, the installer adds it for new shells: it writes
`rmcp-release-demo/env.sh` in your config directory (`$XDG_CONFIG_HOME`, by default `~/.config`) and a line
that sources it to `~/.profile` and your other shell startup files.

- Upgrade: run the same command again.
- Install a specific version: replace `latest/download` with `download/v<version>`, for example

  ```sh
  curl --proto '=https' --tlsv1.2 -LsSf https://github.com/aloong-planet/rmcp-release-demo/releases/download/v0.1.0/rmcp-release-demo-installer.sh | sh
  ```

- Uninstall: delete the binary only.

  ```sh
  rm ~/.local/bin/rmcp-release-demo
  ```

  Leave `rmcp-release-demo/` in your config directory and the lines the installer added to your shell
  startup files in place: they put `~/.local/bin` on your `PATH`, which other tools installed there may rely
  on, and removing the file without the lines makes every new shell report a missing file.

## Use it from an MCP client

Point the client at the installed binary as a stdio server, with its absolute path: the one from the table
above, with `~` written out as in the curl installer section. Many clients do not expand `~`. For example:

```json
{
  "mcpServers": {
    "rmcp-release-demo": {
      "command": "/Users/<you>/.local/bin/rmcp-release-demo"
    }
  }
}
```

The server writes only MCP messages to stdout; its log lines go to stderr.

## License

MIT, see [LICENSE](LICENSE).
