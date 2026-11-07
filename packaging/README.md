# Packaging

`package.sh` builds the release files for one platform (see its header); the Release workflow
runs it on every platform and attaches the files to the GitHub Release. `downloads/` is the
[downloads.powergis.app](https://downloads.powergis.app) site. `docker/` holds the nginx
configuration of the `Dockerfile` at the repository root, which builds the web app into an image
to host yourself (see [Installation](../docs/installation.md#self-host-the-web-app-with-docker)),
and `cli.Dockerfile`, the command line's image. `.github/workflows/docker.yml` publishes both to
ghcr.io.

## Package managers

After each release build, `.github/workflows/package-managers.yml` publishes the release to four
package managers. Each one repackages the release's own files (nothing is rebuilt), takes its
checksums from the release's `SHA256SUMS`, runs as its own job and skips itself when its secret
is missing. Pre-releases are not published. To republish a release, run the workflow by hand
with its tag.

| Channel | Package | Install | Secret |
| --- | --- | --- | --- |
| Homebrew tap [opengeos/homebrew-powergis](https://github.com/opengeos/homebrew-powergis) | cask `powergis` (the DMGs), formula `powergis-cli` (macOS and Linux) | `brew install --cask opengeos/powergis/powergis` | `HOMEBREW_TAP_DEPLOY_KEY` |
| [AUR](https://aur.archlinux.org/packages/powergis-bin) | `powergis-bin` (the .deb files, x86_64 and aarch64) | `yay -S powergis-bin` | `AUR_SSH_PRIVATE_KEY` |
| [Fedora COPR](https://copr.fedorainfracloud.org/coprs/giswqs/powergis/) | `powergis` (the .rpm files plus AppStream metainfo) | `sudo dnf copr enable giswqs/powergis && sudo dnf install powergis` | `COPR_CONFIG` |
| [winget](https://github.com/microsoft/winget-pkgs/tree/master/manifests/o/OpenGeos/PowerGIS) | `OpenGeos.PowerGIS` (setup.exe per user, .msi per machine) | `winget install OpenGeos.PowerGIS` | `WINGET_TOKEN` |

Each folder has the script that writes the package for a version:

- `homebrew/render.sh <version> <SHA256SUMS> <tap folder>` writes `Casks/powergis.rb` and
  `Formula/powergis-cli.rb`.
- `aur/render.sh <version> <SHA256SUMS> > PKGBUILD`.
- `copr/render.sh <version> <date> <folder>` writes `powergis.spec` and
  `org.powergis.PowerGIS.metainfo.xml`. Build the source RPM with both release `.rpm` files in
  the same folder.
- `winget/` holds reference copies of the manifests; winget-releaser writes each new version
  from the previous one in winget-pkgs.

### Trying a package locally

```sh
gh release download v0.1.0 -R opengeos/PowerGIS -p SHA256SUMS
packaging/aur/render.sh 0.1.0 SHA256SUMS > /tmp/aur/PKGBUILD && (cd /tmp/aur && makepkg -si)
packaging/copr/render.sh 0.1.0 2026-10-12 /tmp/copr     # then rpmbuild -bs in Fedora
packaging/homebrew/render.sh 0.1.0 SHA256SUMS /tmp/tap && brew style /tmp/tap/Casks/powergis.rb
```

### Accounts and secrets

- **Homebrew**: the tap repository has a write deploy key; its private half is
  `HOMEBREW_TAP_DEPLOY_KEY`. Taps need `brew trust opengeos/powergis` once on Homebrew 5.1 and
  later. The official homebrew/cask repository can take the cask once the repository meets
  Homebrew's notability rules (75 stars, 30 days old).
- **AUR**: the maintainer account (giswqs) lists the public half of `AUR_SSH_PRIVATE_KEY` under
  *My Account › SSH Public Key*.
- **COPR**: the project `giswqs/powergis` builds for the Fedora x86_64 and aarch64 chroots.
  `COPR_CONFIG` is the whole block from <https://copr.fedorainfracloud.org/api/>; the token
  expires after 180 days, so renew the secret when the COPR job fails to authenticate.
- **winget**: `WINGET_TOKEN` is a classic token with the `public_repo` scope for the account
  owning the `winget-pkgs` fork (giswqs/winget-pkgs). The first version (0.1.0) was submitted
  by hand with `komac submit packaging/winget`.
