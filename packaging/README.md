# bbcpr Package Distribution

This directory contains packaging configurations for distributing bbcpr across major package managers.

## Quick Distribution Status

| Platform | Package Manager | Status | Installation Command |
|----------|----------------|--------|---------------------|
| Arch Linux | AUR | ✅ Ready | `yay -S bbcpr` |
| macOS/Linux | Homebrew | ✅ Ready | `brew install bbcpr` |
| Universal | Snap | ✅ Ready | `snap install bbcpr` |
| Debian/Ubuntu | APT | ✅ Ready | `apt install bbcpr` |
| Fedora/RHEL | DNF/YUM | ✅ Ready | `dnf install bbcpr` |
| Windows | Chocolatey | ✅ Ready | `choco install bbcpr` |
| Universal | Flatpak | 🚧 TODO | `flatpak install bbcpr` |
| Docker | Docker Hub | 🚧 TODO | `docker run 88plug/bbcpr` |

## Package Submission Guide

### Arch Linux (AUR)
```bash
cd packaging/arch
# Create account at https://aur.archlinux.org
git clone ssh://aur@aur.archlinux.org/bbcpr.git
cp PKGBUILD .SRCINFO bbcpr/
cd bbcpr
git add .
git commit -m "Initial release v0.1.0"
git push
```

### Homebrew
```bash
# Fork homebrew-core
# Copy bbcpr.rb to Formula/
# Submit PR to homebrew/homebrew-core with bbcpr repo URL
```

### Snap Store
```bash
cd packaging/snap
snapcraft
snapcraft upload bbcpr_0.1.0_amd64.snap
snapcraft release bbcpr 0.1.0 stable
```

### Debian/Ubuntu
```bash
cd packaging/debian/bbcpr
# Build source package
dpkg-source -b .
# Build binary package
dpkg-buildpackage -us -uc
# Upload to mentors.debian.net for sponsorship
```

### Fedora/RHEL
```bash
cd packaging/rpm
# Create Fedora account
fedpkg --release f39 local
# Submit for review at bugzilla.redhat.com
```

### Chocolatey
```powershell
cd packaging\chocolatey
choco pack
choco push bbcpr.0.1.0.nupkg --source https://push.chocolatey.org/
```

## Building Packages Locally

### Arch
```bash
cd packaging/arch
makepkg -si
```

### Debian
```bash
cd packaging/debian/bbcpr
dpkg-buildpackage -b
```

### RPM
```bash
cd packaging/rpm
rpmbuild -ba bbcpr.spec
```

### Snap
```bash
cd packaging/snap
snapcraft
```

## Automated Building

See `.github/workflows/package-release.yml` for automated package building on release.