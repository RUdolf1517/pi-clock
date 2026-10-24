# Changelog

Notable changes to pi-clock are recorded here. Versions follow Semantic Versioning (`MAJOR.MINOR.PATCH`).

## [Unreleased]

## [0.1.1] - 2026-10-24

- Fix TTY detection and inherited stdin for the Ghostty/Unix TUI.

## [0.1.0] - 2026-10-24

- Consolidate app commands under the `pi-clock` CLI.
- Add cross-platform CI and tag-driven release builds.
- Add an interactive TUI, a 12-hour smooth dial, and one-command installers.
- Improve fraction arithmetic and share platform local-time handling.
- Correct the 03:30:45 example: its combined coefficient is 281/480, not 221/480.
