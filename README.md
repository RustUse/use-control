# use-control

Composable primitive control-system utilities for Rust.

`use-control` is part of RustUse, alongside sibling repositories such as
`use-math`, `use-stats`, `use-optimization`, `use-simulation`, `use-color`,
`use-text`, `use-time`, and `use-units`. It groups small, focused crates for
feedback loops, PID controllers, setpoints, error calculations, control
signals, system response helpers, and simple stability-oriented utilities.

The RustUse approach in this workspace stays intentionally narrow:

- crates stay small and independently useful
- APIs stay explicit, documented, tested, and composable
- implementations favor practical `f64` and `usize` helpers over framework-style abstractions
- dependencies stay minimal so each crate is easy to audit and adopt

## Workspace crates

- `use-control`: thin facade crate that reexports the full control workspace
- `use-feedback`: primitive positive and negative feedback helpers
- `use-pid`: minimal PID gains, state, and controller updates
- `use-setpoint`: setpoint and tolerance checks
- `use-control-error`: control error, relative error, percent error, and deadband helpers
- `use-control-signal`: control signal validation, clamping, and saturation helpers
- `use-system-response`: first-order response functions and sampling helpers
- `use-stability`: gain classification, boundedness, and settling-time helpers

## Facade crate

If you want one dependency for the whole workspace, use `use-control`. It
reexports each focused crate and exposes the focused APIs directly so this
works:

```rust
use use_control::*;

let mut controller = PidController::new(PidGains {
    kp: 2.0,
    ki: 0.5,
    kd: 0.0,
})
.unwrap();
let output = controller.update(10.0, 8.0, 0.5).unwrap();
let signal = ControlSignal::new(output).unwrap().clamp(0.0, 10.0).unwrap();

assert!(signal.value() >= 0.0);
assert!(within_tolerance(10.0, 9.9, 0.2).unwrap());
```

## Status

This workspace is experimental while it remains below `0.3.0`. Expect the
public API to stay small and practical, but still evolve as the RustUse
control surface becomes clearer.

## Development

Run the standard workspace checks from the repository root:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo doc --workspace --no-deps
```
