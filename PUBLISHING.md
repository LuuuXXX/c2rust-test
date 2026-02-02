# Publishing to crates.io

This document describes how to publish the `c2rust-test` crate to crates.io using the automated GitHub Actions workflow.

## Prerequisites

### 1. Set up GitHub Secret

Before you can publish to crates.io, you need to add your crates.io API token to GitHub Secrets:

1. Obtain your crates.io API token:
   - Log in to [crates.io](https://crates.io)
   - Go to your [Account Settings](https://crates.io/settings/tokens)
   - Click "New Token"
   - Give it a name (e.g., "github-actions")
   - Copy the generated token

2. Add the token to GitHub Secrets:
   - Go to the repository on GitHub
   - Navigate to Settings → Secrets and variables → Actions
   - Click "New repository secret"
   - Name: `CRATES_IO_TOKEN`
   - Value: Paste your crates.io API token
   - Click "Add secret"

## Publishing a New Version

### Automatic Publishing (via Git Tag)

The recommended way to publish is by creating and pushing a version tag:

```bash
# Create a tag (format: v<major>.<minor>.<patch>)
git tag v0.1.0

# Push the tag to GitHub
git push origin v0.1.0
```

This will automatically trigger the GitHub Actions workflow, which will:
1. Run all tests
2. Publish the crate to crates.io

### Manual Publishing (via workflow_dispatch)

You can also manually trigger the publish workflow:

1. Go to the repository on GitHub
2. Navigate to Actions → Publish to crates.io
3. Click "Run workflow"
4. Select the branch and click "Run workflow"

## Version Updates

Before creating a new tag for publishing:

1. Update the version number in `Cargo.toml`:
   ```toml
   [package]
   version = "0.2.0"  # Update to new version
   ```

2. Commit the change:
   ```bash
   git add Cargo.toml
   git commit -m "Bump version to 0.2.0"
   git push
   ```

3. Create and push the tag:
   ```bash
   git tag v0.2.0
   git push origin v0.2.0
   ```

## Important Notes

- **First-time publishing**: The first time you publish a crate, the name will be reserved permanently on crates.io
- **Crate name availability**: Ensure the crate name is available on crates.io before publishing
- **Version immutability**: Once a version is published to crates.io, it cannot be changed or deleted
- **Semantic versioning**: Follow [semantic versioning](https://semver.org/) guidelines (MAJOR.MINOR.PATCH)
- **Token security**: Never commit the crates.io token to the repository - always use GitHub Secrets

## Troubleshooting

### Workflow fails with authentication error
- Verify that `CRATES_IO_TOKEN` is correctly set in GitHub Secrets
- Ensure the token has not expired
- Generate a new token if necessary

### Workflow fails with "crate name already taken"
- The crate name is already in use on crates.io
- Choose a different name in `Cargo.toml` and try again

### Workflow fails with test errors
- Fix the failing tests before publishing
- Run `cargo test` locally to verify all tests pass
