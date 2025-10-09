# Release Process

This document describes how to release a new version of `rustedbytes-nmea` to GitHub and crates.io.

## Prerequisites

Before creating a release, ensure:

1. All changes for the release are merged to the `main` branch
2. All tests pass on the `main` branch
3. You have determined the appropriate version number following [Semantic Versioning](https://semver.org/)

### Semantic Versioning Quick Reference

- **Major version** (X.0.0): Breaking API changes
- **Minor version** (0.X.0): New features, backward compatible
- **Patch version** (0.0.X): Bug fixes, backward compatible

## Release Workflow

The release process is automated using GitHub Actions. The workflow will:

1. ✅ Validate the version format
2. ✅ Run all tests, formatting checks, and linting
3. ✅ Update version in `Cargo.toml`
4. ✅ Update version in `README.md`
5. ✅ Create or update `CHANGELOG.md`
6. ✅ Commit changes and create a git tag
7. ✅ Create a GitHub Release
8. ✅ Publish to crates.io

## How to Release

### Step 1: Navigate to Actions

1. Go to the [Actions tab](https://github.com/mad4j/rustedbytes-nmea/actions) in the repository
2. Select the "Release" workflow from the left sidebar

### Step 2: Trigger the Workflow

1. Click the "Run workflow" button
2. Enter the version number (e.g., `0.1.1`, `0.2.0`, `1.0.0`)
   - Format must be `X.Y.Z` (three numbers separated by periods)
   - Do not include the `v` prefix
3. Click "Run workflow" to start the release process

### Step 3: Monitor the Workflow

The workflow will run through several jobs:

1. **validate-version**: Checks that the version format is correct
2. **test**: Runs all tests and quality checks
3. **prepare-release**: Updates files and creates git tag
4. **create-github-release**: Creates the GitHub release
5. **publish-crates-io**: Publishes to crates.io

If any step fails, the workflow will stop and you can check the logs to see what went wrong.

## Required Secrets

The workflow requires the following secret to be configured in the repository:

- **`CARGO_REGISTRY_TOKEN`**: Token for publishing to crates.io
  - Obtain from [crates.io account settings](https://crates.io/settings/tokens)
  - Add to repository secrets at: Settings → Secrets and variables → Actions → New repository secret

## Manual CHANGELOG Updates

While the workflow automatically creates/updates the CHANGELOG, you may want to manually edit it before release:

1. Edit `CHANGELOG.md` to add more detailed release notes
2. Commit your changes to `main`
3. Then run the release workflow

The workflow will preserve your manual additions and just add the version header if needed.

## Post-Release Checklist

After the release completes successfully:

- [ ] Verify the new version appears on [crates.io](https://crates.io/crates/rustedbytes-nmea)
- [ ] Check the [GitHub Releases page](https://github.com/mad4j/rustedbytes-nmea/releases)
- [ ] Verify the documentation on [docs.rs](https://docs.rs/rustedbytes-nmea)
- [ ] Update any external documentation or announcements

## Troubleshooting

### Version Validation Failed

**Error**: "Version must be in format X.Y.Z"

**Solution**: Ensure you're using the correct format (e.g., `0.1.1`, not `v0.1.1` or `0.1`)

### Tests Failed

**Error**: Test failures in the `test` job

**Solution**: Fix the failing tests on `main` branch before attempting release

### Publish to crates.io Failed

**Error**: "failed to authenticate to registry"

**Solution**: Ensure `CARGO_REGISTRY_TOKEN` secret is correctly configured

**Error**: "crate version X.Y.Z already exists"

**Solution**: You cannot republish the same version. Increment the version number.

### Git Push Failed

**Error**: Permission denied or authentication failed

**Solution**: Ensure the workflow has `contents: write` permission (already configured)

## Emergency Rollback

If you need to unpublish a release:

1. **GitHub Release**: You can delete the release from the GitHub Releases page
2. **Git Tag**: You can delete the tag with `git push --delete origin vX.Y.Z`
3. **crates.io**: Once published, versions cannot be unpublished, but you can yank them:
   ```bash
   cargo yank --version X.Y.Z
   ```

Note: Yanking prevents new projects from using the version but doesn't remove it.

## Release Cadence

There is no fixed release schedule. Releases are made when:

- Critical bugs are fixed
- Significant new features are added
- Multiple small improvements have accumulated

Maintain a balance between releasing frequently enough to get fixes to users and allowing enough time for changes to be tested.
