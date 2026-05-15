#![forbid(unsafe_code)]
//! Setpoint helpers for simple control loops.
//!
//! The crate keeps setpoints explicit by storing only a target and a tolerance.
//!
//! # Examples
//!
//! ```rust
//! use use_setpoint::{within_tolerance, Setpoint};
//!
//! let setpoint = Setpoint::new(10.0, 0.2).unwrap();
//! assert!(setpoint.is_reached(9.9));
//! assert!((setpoint.error(9.8) - 0.2).abs() < 1e-12);
//! assert!(within_tolerance(10.0, 10.1, 0.2).unwrap());
//! ```

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Setpoint {
    pub target: f64,
    pub tolerance: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SetpointError {
    InvalidTarget,
    InvalidTolerance,
    InvalidMeasured,
}

impl Setpoint {
    pub fn new(target: f64, tolerance: f64) -> Result<Self, SetpointError> {
        if !target.is_finite() {
            return Err(SetpointError::InvalidTarget);
        }

        if !tolerance.is_finite() || tolerance < 0.0 {
            return Err(SetpointError::InvalidTolerance);
        }

        Ok(Self { target, tolerance })
    }

    pub fn is_reached(&self, measured: f64) -> bool {
        measured.is_finite() && (self.target - measured).abs() <= self.tolerance
    }

    pub fn error(&self, measured: f64) -> f64 {
        self.target - measured
    }
}

pub fn within_tolerance(target: f64, measured: f64, tolerance: f64) -> Result<bool, SetpointError> {
    let setpoint = Setpoint::new(target, tolerance)?;
    if !measured.is_finite() {
        return Err(SetpointError::InvalidMeasured);
    }

    Ok(setpoint.is_reached(measured))
}

#[cfg(test)]
mod tests {
    use super::{within_tolerance, Setpoint, SetpointError};

    #[test]
    fn checks_tolerance_and_error() {
        let setpoint = Setpoint::new(10.0, 0.2).unwrap();

        assert!(setpoint.is_reached(9.9));
        assert!(!setpoint.is_reached(9.6));
        assert_eq!(setpoint.error(9.5), 0.5);
        assert!(within_tolerance(10.0, 10.1, 0.2).unwrap());
    }

    #[test]
    fn rejects_invalid_inputs() {
        assert_eq!(
            Setpoint::new(f64::NAN, 0.1),
            Err(SetpointError::InvalidTarget)
        );
        assert_eq!(
            Setpoint::new(1.0, -0.1),
            Err(SetpointError::InvalidTolerance)
        );
        assert_eq!(
            within_tolerance(1.0, f64::NAN, 0.1),
            Err(SetpointError::InvalidMeasured)
        );
    }
}
