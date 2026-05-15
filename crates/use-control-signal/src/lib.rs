#![forbid(unsafe_code)]
//! Control signal primitives.
//!
//! The crate keeps control signal transformations explicit: construct, clamp,
//! scale, or saturate a finite scalar value.
//!
//! # Examples
//!
//! ```rust
//! use use_control_signal::{clamp_signal, saturate, ControlSignal};
//!
//! let signal = ControlSignal::new(12.0).unwrap().clamp(0.0, 10.0).unwrap();
//! assert_eq!(signal.value(), 10.0);
//! assert_eq!(clamp_signal(2.5, 0.0, 2.0).unwrap(), 2.0);
//! assert_eq!(saturate(-5.0, 2.0).unwrap(), -2.0);
//! ```

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ControlSignal {
    value: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlSignalError {
    InvalidValue,
    InvalidBounds,
    InvalidFactor,
}

impl ControlSignal {
    pub fn new(value: f64) -> Result<Self, ControlSignalError> {
        if !value.is_finite() {
            return Err(ControlSignalError::InvalidValue);
        }

        Ok(Self { value })
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn clamp(self, min: f64, max: f64) -> Result<Self, ControlSignalError> {
        Self::new(clamp_signal(self.value, min, max)?)
    }

    pub fn scale(self, factor: f64) -> Result<Self, ControlSignalError> {
        if !factor.is_finite() {
            return Err(ControlSignalError::InvalidFactor);
        }

        let value = self.value * factor;
        Self::new(value)
    }
}

pub fn clamp_signal(value: f64, min: f64, max: f64) -> Result<f64, ControlSignalError> {
    if !value.is_finite() || !min.is_finite() || !max.is_finite() {
        return Err(ControlSignalError::InvalidValue);
    }

    if min > max {
        return Err(ControlSignalError::InvalidBounds);
    }

    Ok(value.clamp(min, max))
}

pub fn saturate(value: f64, limit: f64) -> Result<f64, ControlSignalError> {
    if !limit.is_finite() || limit < 0.0 {
        return Err(ControlSignalError::InvalidBounds);
    }

    clamp_signal(value, -limit, limit)
}

#[cfg(test)]
mod tests {
    use super::{ControlSignal, ControlSignalError, clamp_signal, saturate};

    #[test]
    fn clamps_and_scales_signals() {
        let signal = ControlSignal::new(12.0).unwrap().clamp(0.0, 10.0).unwrap();

        assert_eq!(signal.value(), 10.0);
        assert_eq!(signal.scale(0.5).unwrap().value(), 5.0);
        assert_eq!(clamp_signal(-2.0, 0.0, 3.0).unwrap(), 0.0);
    }

    #[test]
    fn saturates_signals() {
        assert_eq!(saturate(7.0, 5.0).unwrap(), 5.0);
        assert_eq!(saturate(-7.0, 5.0).unwrap(), -5.0);
    }

    #[test]
    fn rejects_invalid_values() {
        assert_eq!(
            ControlSignal::new(f64::NAN),
            Err(ControlSignalError::InvalidValue)
        );
        assert_eq!(
            clamp_signal(1.0, 2.0, 1.0),
            Err(ControlSignalError::InvalidBounds)
        );
        assert_eq!(
            ControlSignal::new(1.0).unwrap().scale(f64::NAN),
            Err(ControlSignalError::InvalidFactor)
        );
        assert_eq!(saturate(1.0, -1.0), Err(ControlSignalError::InvalidBounds));
    }
}
