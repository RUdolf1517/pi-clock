# Releases and package managers

`Cargo.toml` is the single source of truth for the SemVer version. `pi-clock version` prints the binary version, and tags must match `v<version>` exactly.

## Cut a release

1. Update `version` in `Cargo.toml` and the `pi-clock` package entry in `Cargo.lock` together. Add the release notes to `CHANGELOG.md`.
2. Run `cargo test --locked --no-default-features`, review, and merge the changes into `master`.
3. From the merged commit, create and push the matching tag, for example:

   ```sh
   git tag v0.2.0
   git push origin v0.2.0
   ```

4. GitHub Actions verifies the tag against Cargo, builds platform archives and checksums, creates Linux `.deb` and `.rpm` packages, and publishes all release assets.
5. After the release completes, update `Formula/pi-clock.rb` to the new version and copy the archive SHA-256 values from the release's `SHA256SUMS`. Commit the formula update to `master`.
6. Copy the three manifests in `packaging/winget/<version>/` to a PR in `microsoft/winget-pkgs` under `manifests/r/RUdolf1517/pi-clock/<version>/`. The repository includes the manifests for the first supported release; update their version, URL, release date, and Windows zip SHA-256 for each release.

## Install

### Homebrew

This repository is a third-party tap (it is not named `homebrew-pi-clock`), so specify its GitHub URL:

```sh
brew tap RUdolf1517/pi-clock https://github.com/RUdolf1517/pi-clock
brew install RUdolf1517/pi-clock/pi-clock
```

### Debian / Ubuntu (apt)

Download the matching architecture package from the [latest release](https://github.com/RUdolf1517/pi-clock/releases/latest), then install it:

```sh
sudo apt install ./pi-clock_amd64.deb
```

Use `pi-clock_arm64.deb` on ARM64.

### Fedora (dnf)

Download the matching RPM from the [latest release](https://github.com/RUdolf1517/pi-clock/releases/latest), then install it:

```sh
sudo dnf install ./pi-clock.x86_64.rpm
```

Use `pi-clock.aarch64.rpm` on ARM64.

### Windows (winget)

Once the manifest PR is accepted into the community repository:

```powershell
winget install -e --id RUdolf1517.pi-clock
```

Until then, install the local manifest from a clone of this repository (enable `LocalManifestFiles` once in an elevated terminal):

```powershell
git clone https://github.com/RUdolf1517/pi-clock
winget settings --enable LocalManifestFiles
winget install --manifest .\packaging\winget\0.1.2
```

### Cargo

Cargo users can install directly from the Git repository without a prebuilt binary:

```sh
cargo install --git https://github.com/RUdolf1517/pi-clock --tag v0.1.2
```

On macOS, add `--features tray-app` if the tray application should be included.
