# Maintainer Release Flow

This document describes the maintainer release flow for `use-control`.

It covers two different paths:

- the initial public crates.io rollout, which is staged and manual
- normal follow-up releases, where version bumps and changelog generation are
  automated and publishing stays maintainer-triggered

## Current model

- `Release PR Automation` opens or updates a release PR from `main`.
- `release-plz` keeps every publishable crate in one lockstep version group.
- The shared root `CHANGELOG.md` is generated through the `use-control` package
  entry and includes focused-crate commits.
- `Release Publish Automation` is post-initial-release only and remains guarded.

## Initial public release flow

Use this exact staged order for the initial crates.io rollout:

1. Run the focused-crate publish-readiness checks and publish:
   - `use-feedback`
   - `use-pid`
   - `use-setpoint`
   - `use-control-error`
   - `use-control-signal`
   - `use-system-response`
   - `use-stability`
2. Wait for crates.io propagation.
3. Run the facade publish-readiness workflow.
4. Publish `use-control`.

## Normal post-initial-release flow

1. Merge ordinary PRs into `main` with conventional commit style in the final
   commit subject or squash-merge title.
2. Let `Release PR Automation` open or update the release PR.
3. Review the release PR for the lockstep version bump and generated
   `CHANGELOG.md`.
4. Clean up the changelog directly in the release PR branch when the generated
   wording is accurate but not maintainer quality.
5. Merge the release PR into `main`.
6. Let `Release Publish Automation` publish from the merged release commit, or
   manually dispatch it with `post-initial-release = true` if you need a
   controlled rerun.

## One-time post-initial-release setup

Before relying on automated publishing:

- configure crates.io Trusted Publishing for every published crate with
  repository owner `RustUse`, repository name `use-control`, and workflow
  filename `release-plz-release.yml`
- set the repository variable `CRATES_IO_AUTOPUBLISH_ENABLED` to `true` only
  after the initial manual crates.io wave is complete
- do not configure `CARGO_REGISTRY_TOKEN` for the trusted publishing workflow
