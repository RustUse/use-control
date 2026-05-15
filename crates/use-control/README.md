# use-control

Composable facade crate for RustUse control-system primitives.

## Install

```toml
[dependencies]
use-control = "0.0.1"
```

## What it reexports

`use-control` reexports `use-feedback`, `use-pid`, `use-setpoint`,
`use-control-error`, `use-control-signal`, `use-system-response`, and
`use-stability`.

## When to use it

Use this crate when you want one dependency for the full control workspace.
Depend on the focused crates directly when you only need a smaller slice.

## Status

This crate is experimental and may evolve while the workspace remains below
`0.3.0`.