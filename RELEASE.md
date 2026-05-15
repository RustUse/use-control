# Release Policy

RustUse/use-control is not published yet. The workspace keeps `publish = false`
at the root package level while the intentional first-wave crate manifests opt
into `publish = true`.

This workspace does not have internal dependencies between focused crates, so
the initial public crates.io rollout has two waves instead of three.

## First Publish Waves

### Wave 1: Focused crates

Publish these crates first:

- `use-feedback`
- `use-pid`
- `use-setpoint`
- `use-control-error`
- `use-control-signal`
- `use-system-response`
- `use-stability`

These crates do not depend on other publishable crates from the same
workspace.

### Wave 2: Facade crate

After every focused crate from Wave 1 is visible on crates.io, publish:

- `use-control`

The facade dry-run is intentionally separate because it depends on the full
focused crate surface being available in the registry.

## Publish Surface

Before the first publish wave, confirm that the release surface:

- keeps the workspace-level default at `publish = false`
- keeps the focused crates at `publish = true`
- keeps `crates/use-control/Cargo.toml` at `publish = true`
- leaves any future non-release crates opted out until intentionally reviewed

## Versioning

- The workspace uses lockstep `0.x.y` versioning.
- Before `1.0`, breaking changes should bump the minor version.
- Before `1.0`, additive compatible changes should bump the patch version.
- The facade crate should only advertise the focused crates that are actively
  supported.

## Automated Release Validation

The repository includes a staged release-validation path:

- `.github/workflows/publish-readiness.yml` runs on pull requests, pushes to
  `main`, and manual dispatch for all focused crates.
- `.github/workflows/facade-publish-readiness.yml` is the final manual facade
  check after all focused crates are live on crates.io.
- `.github/workflows/release-plz-pr.yml` prepares lockstep version bumps and
  changelog updates.
- `.github/workflows/release-plz-release.yml` remains post-initial-release
  automation only and is guarded behind `CRATES_IO_AUTOPUBLISH_ENABLED=true` or
  manual dispatch.

## One-time post-initial-release setup

Before relying on automated publishing:

- Configure crates.io Trusted Publishing for every published crate with
  repository owner `RustUse`, repository name `use-control`, and workflow
  filename `release-plz-release.yml`.
- Leave the crates.io environment field empty unless you intentionally add a
  matching GitHub Actions environment later.
- Set the repository variable `CRATES_IO_AUTOPUBLISH_ENABLED` to `true` only
  after the initial manual crates.io wave is complete.
- Do not set `CARGO_REGISTRY_TOKEN` for the release-plz workflow when using
  trusted publishing.

## Maintainer Release Checklist

For the initial public release:

1. Run the full quality checks and focused-crate publish-readiness flow.
2. Publish the focused crates manually.
3. Wait for crates.io index propagation.
4. Run the facade publish-readiness flow.
5. Publish `use-control`.

For normal post-initial-release releases:

1. Merge ordinary PRs with clean conventional commit style in the final commit
   subject or squash title.
2. Let `Release PR Automation` open or update the release PR.
3. Review the lockstep version bump and generated `CHANGELOG.md`.
4. Merge the release PR.
5. Let `Release Publish Automation` publish from the merged release commit, or
   manually dispatch it with `post-initial-release = true` if needed.

## Publish Readiness Checklist

1. Confirm `cargo fmt --all -- --check` is clean.
2. Confirm `cargo clippy --workspace --all-targets -- -D warnings` passes.
3. Confirm `cargo test --workspace` passes.
4. Confirm `cargo doc --workspace --no-deps` passes.
5. Confirm focused-crate dry-runs pass before the first publish attempt.
6. Confirm the facade dry-run passes only after all focused crates are visible
   on crates.io.
