# Changelog

Notable changes to pi-clock are recorded here. Versions follow Semantic Versioning (`MAJOR.MINOR.PATCH`).

## [Unreleased]

## [0.1.3] - 2026-10-24

- Add Homebrew, apt, dnf, and winget package-manager distribution alongside release archives.

## [0.1.2] - 2026-10-24

- Redesign the TUI dial with Unicode hands, hour markers, and colored layers.
- Start the tray alongside the TUI in tray-enabled builds; add a TUI-only option.

## [0.1.1] - 2026-10-24

- Fix TTY detection and inherited stdin for the Ghostty/Unix TUI.

## [0.1.0] - 2026-10-24

- Consolidate app commands under the `pi-clock` CLI.
- Add cross-platform CI and tag-driven release builds.
- Add an interactive TUI, a 12-hour smooth dial, and one-command installers.
- Improve fraction arithmetic and share platform local-time handling.
- Correct the 03:30:45 example: its combined coefficient is 281/480, not 221/480.
