#![forbid(unsafe_code)]
//! Control error calculation primitives.
//!
//! The crate provides a narrow set of helpers over scalar `f64` control error
//! values.
//!
//! # Examples
//!
//! ```rust
//! use use_control_error::{absolute_error, deadband, percent_error};
//!
//! assert_eq!(absolute_error(10.0, 9.0), 1.0);
//! assert_eq!(deadband(0.05, 0.1).unwrap(), 0.0);
//! assert_eq!(percent_error(10.0, 8.0), Some(20.0));
//! ```

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlErrorError {
    InvalidError,
    InvalidThreshold,
}

pub fn error(setpoint: f64, measured: f64) -> f64 {
    setpoint - measured
}

pub fn absolute_error(setpoint: f64, measured: f64) -> f64 {
    error(setpoint, measured).abs()
}

pub fn relative_error(setpoint: f64, measured: f64) -> Option<f64> {
    if !setpoint.is_finite() || !measured.is_finite() || setpoint == 0.0 {
        return None;
    }

    Some(error(setpoint, measured) / setpoint)
}

pub fn percent_error(setpoint: f64, measured: f64) -> Option<f64> {
    relative_error(setpoint, measured).map(|value| value * 100.0)
}

pub fn deadband(error: f64, threshold: f64) -> Result<f64, ControlErrorError> {
    if !error.is_finite() {
        return Err(ControlErrorError::InvalidError);
    }

    if !threshold.is_finite() || threshold < 0.0 {
        return Err(ControlErrorError::InvalidThreshold);
    }

    if error.abs() <= threshold {
        Ok(0.0)
    } else {
        Ok(error)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ControlErrorError, absolute_error, deadband, error, percent_error, relative_error,
    };

    #[test]
    fn computes_error_forms() {
        assert_eq!(error(10.0, 8.0), 2.0);
        assert_eq!(absolute_error(10.0, 8.0), 2.0);
        assert_eq!(relative_error(10.0, 8.0), Some(0.2));
        assert_eq!(percent_error(10.0, 8.0), Some(20.0));
    }

    #[test]
    fn applies_deadband() {
        assert_eq!(deadband(0.05, 0.1).unwrap(), 0.0);
        assert_eq!(deadband(-0.2, 0.1).unwrap(), -0.2);
    }

    #[test]
    fn rejects_invalid_values() {
        assert_eq!(relative_error(0.0, 1.0), None);
        assert_eq!(percent_error(f64::NAN, 1.0), None);
        assert_eq!(
            deadband(f64::NAN, 0.1),
            Err(ControlErrorError::InvalidError)
        );
        assert_eq!(
            deadband(0.1, -0.1),
            Err(ControlErrorError::InvalidThreshold)
        );
    }
}
