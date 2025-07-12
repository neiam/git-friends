# Git Friends Release Process

This document describes the release process for Git Friends, including automated GitHub Actions releases and manual release management.

## Overview

Git Friends uses GitHub Actions to automatically build and release binaries for multiple platforms when a version tag is pushed. The release process includes:

1. **Cross-platform builds** for Linux, macOS, and Windows
2. **Automated release creation** with pre-built binaries
3. **Docker image publishing** to GitHub Container Registry
4. **Release notes generation** with installation instructions

## Supported Platforms

The release process builds binaries for the following platforms:

- **Linux x86_64** (GNU and musl)
- **Linux ARM64** (aarch64)
- **macOS x86_64** (Intel)
- **macOS ARM64** (Apple Silicon)
- **Windows x86_64**

## Quick Release

For quick releases, use the provided release script:

```bash
# Show current version
./release.sh current

# Create a patch release (0.1.0 -> 0.1.1)
./release.sh patch

# Create a minor release (0.1.0 -> 0.2.0)
./release.sh minor

# Create a major release (0.1.0 -> 1.0.0)
./release.sh major

# Push the release to trigger GitHub Actions
./release.sh push <version>
```

## Manual Release Process

### 1. Pre-release Checklist

Before creating a release, ensure:

- [ ] All tests are passing: `cargo test`
- [ ] Code is properly formatted: `cargo fmt --check`
- [ ] No clippy warnings: `cargo clippy -- -D warnings`
- [ ] Documentation is up to date
- [ ] All changes are committed
- [ ] Working directory is clean
- [ ] You're on the master/main branch

Run the checklist command:
```bash
./release.sh checklist
```

### 2. Update Version

Update the version in `Cargo.toml`:

```toml
[package]
name = "git-friends"
version = "0.2.0"  # Update this
edition = "2021"
```

### 3. Commit Version Change

```bash
git add Cargo.toml
git commit -m "Bump version to 0.2.0"
```

### 4. Create Git Tag

```bash
git tag -a v0.2.0 -m "Release v0.2.0"
```

### 5. Push Release

```bash
git push origin master
git push origin v0.2.0
```

### 6. Monitor GitHub Actions

1. Go to your repository's **Actions** tab
2. Watch the **Rust CI and Docker Build** workflow
3. The workflow will:
   - Run tests and build checks
   - Build binaries for all platforms
   - Create a GitHub release
   - Upload release binaries
   - Push Docker images to GHCR

## Automated Release Workflow

The GitHub Actions workflow (`.github/workflows/rust-ci.yml`) includes three jobs:

### 1. Build Job
- Runs on every push and pull request
- Performs Rust tests, formatting, and clippy checks
- Builds and pushes Docker images to GHCR

### 2. Release Job
- Runs only on version tags (`v*`)
- Builds release binaries for all supported platforms
- Uses cross-compilation for different architectures
- Strips binaries to reduce size
- Creates platform-specific archives (.tar.gz for Unix, .zip for Windows)

### 3. Create Release Job
- Runs after the release job completes
- Downloads all platform binaries
- Generates comprehensive release notes
- Creates GitHub release with all binaries attached

## Release Assets

Each release includes:

### Binaries
- `git-friends-x86_64-linux-gnu.tar.gz`
- `git-friends-x86_64-linux-musl.tar.gz`
- `git-friends-aarch64-linux-gnu.tar.gz`
- `git-friends-x86_64-windows.zip`
- `git-friends-x86_64-macos.tar.gz`
- `git-friends-aarch64-macos.tar.gz`

### Docker Images
- `ghcr.io/your-username/git-friends:latest`
- `ghcr.io/your-username/git-friends:v0.2.0`

### Included Executables
Each archive contains:
- `gf-hook` - Git hook for commit message enhancement
- `gf-server` - Main Git Friends server
- `gf-irc` - IRC client component
- `gf-tester` - Testing and validation tool

## Release Notes

Release notes are automatically generated and include:

- Description of included binaries
- Supported platforms
- Installation instructions
- Docker image information
- Link to full changelog (if GitHub's auto-generated notes)

## Version Naming

Follow [Semantic Versioning](https://semver.org/):

- **MAJOR.MINOR.PATCH** (e.g., 1.0.0)
- **MAJOR**: Breaking changes
- **MINOR**: New features (backwards compatible)
- **PATCH**: Bug fixes (backwards compatible)

### Pre-release Versions

For pre-release versions, use suffixes:
- `1.0.0-alpha.1`
- `1.0.0-beta.1`
- `1.0.0-rc.1`

Pre-release versions are automatically marked as "pre-release" in GitHub.

## Troubleshooting

### Build Failures

If cross-compilation fails:

1. Check that all dependencies support the target platform
2. Verify cross-compilation tools are properly installed
3. Review the GitHub Actions logs for specific errors

### Missing Binaries

If some platform binaries are missing:

1. Check the matrix strategy in `.github/workflows/rust-ci.yml`
2. Verify the target platform is properly configured
3. Check for platform-specific build failures

### Release Creation Issues

If the release creation fails:

1. Verify the `GITHUB_TOKEN` has proper permissions
2. Check that the tag follows the `v*` pattern
3. Ensure the tag is properly pushed to the repository

## Local Testing

To test the release process locally:

```bash
# Test version update (dry run)
./release.sh prepare 0.2.0 --dry-run

# Test cross-compilation locally
cargo build --release --target x86_64-unknown-linux-musl

# Test Docker build
docker build -t git-friends:test .
```

## Security Considerations

- Release binaries are built in GitHub's secure environment
- Docker images are signed and pushed to GHCR
- All release assets are checksummed by GitHub
- No secrets or credentials are embedded in binaries

## Post-Release

After a successful release:

1. Verify all binaries work on their target platforms
2. Test the Docker image
3. Update documentation if needed
4. Announce the release (if applicable)
5. Monitor for any issues reported by users

## Example Release Workflow

Here's a complete example of releasing version 0.2.0:

```bash
# 1. Check current status
./release.sh current
./release.sh checklist

# 2. Prepare the release
./release.sh prepare 0.2.0

# 3. Push to trigger GitHub Actions
./release.sh push 0.2.0

# 4. Monitor GitHub Actions
# Visit https://github.com/your-username/git-friends/actions

# 5. Verify release
# Visit https://github.com/your-username/git-friends/releases
```

This process ensures consistent, automated releases with comprehensive platform support and proper documentation.
