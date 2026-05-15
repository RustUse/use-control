#![forbid(unsafe_code)]
//! Simple stability-oriented helpers.
//!
//! The crate provides scalar classification helpers and a settling-time helper
//! over sampled response values.
//!
//! # Examples
//!
//! ```rust
//! use use_stability::{classify_gain, is_bounded, settling_time, Stability};
//!
//! assert_eq!(classify_gain(0.8, 1.0).unwrap(), Stability::Stable);
//! assert!(is_bounded(&[0.5, -0.5, 0.25], 1.0).unwrap());
//! assert_eq!(settling_time(&[1.4, 1.05, 1.01, 1.0], 1.0, 0.05, 0.5).unwrap(), Some(0.5));
//! ```

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stability {
    Stable,
    Unstable,
    Marginal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StabilityError {
    InvalidGain,
    InvalidLimit,
    InvalidValues,
    InvalidBound,
    InvalidTarget,
    InvalidTolerance,
    InvalidTimestep,
}

pub fn classify_gain(gain: f64, stable_limit: f64) -> Result<Stability, StabilityError> {
    if !gain.is_finite() {
        return Err(StabilityError::InvalidGain);
    }

    if !stable_limit.is_finite() || stable_limit <= 0.0 {
        return Err(StabilityError::InvalidLimit);
    }

    let magnitude = gain.abs();
    if magnitude < stable_limit {
        Ok(Stability::Stable)
    } else if magnitude > stable_limit {
        Ok(Stability::Unstable)
    } else {
        Ok(Stability::Marginal)
    }
}

pub fn is_bounded(values: &[f64], bound: f64) -> Result<bool, StabilityError> {
    if !bound.is_finite() || bound <= 0.0 {
        return Err(StabilityError::InvalidBound);
    }

    if values.iter().any(|value| !value.is_finite()) {
        return Err(StabilityError::InvalidValues);
    }

    Ok(values.iter().all(|value| value.abs() <= bound))
}

pub fn settling_time(
    values: &[f64],
    target: f64,
    tolerance: f64,
    dt: f64,
) -> Result<Option<f64>, StabilityError> {
    if !target.is_finite() {
        return Err(StabilityError::InvalidTarget);
    }

    if !tolerance.is_finite() || tolerance < 0.0 {
        return Err(StabilityError::InvalidTolerance);
    }

    if !dt.is_finite() || dt <= 0.0 {
        return Err(StabilityError::InvalidTimestep);
    }

    if values.iter().any(|value| !value.is_finite()) {
        return Err(StabilityError::InvalidValues);
    }

    for start_index in 0..values.len() {
        if values[start_index..]
            .iter()
            .all(|value| within_tolerance(*value, target, tolerance))
        {
            return Ok(Some(start_index as f64 * dt));
        }
    }

    Ok(None)
}

fn within_tolerance(value: f64, target: f64, tolerance: f64) -> bool {
    let scale = 1.0_f64
        .max(value.abs())
        .max(target.abs())
        .max(tolerance.abs());

    (value - target).abs() <= tolerance + f64::EPSILON * scale * 8.0
}

#[cfg(test)]
mod tests {
    use super::{Stability, StabilityError, classify_gain, is_bounded, settling_time};

    #[test]
    fn classifies_gain_magnitude() {
        assert_eq!(classify_gain(0.8, 1.0).unwrap(), Stability::Stable);
        assert_eq!(classify_gain(1.0, 1.0).unwrap(), Stability::Marginal);
        assert_eq!(classify_gain(1.2, 1.0).unwrap(), Stability::Unstable);
    }

    #[test]
    fn detects_bounded_series() {
        assert!(is_bounded(&[0.5, -0.5, 0.25], 1.0).unwrap());
        assert!(!is_bounded(&[0.5, 1.5], 1.0).unwrap());
    }

    #[test]
    fn computes_settling_time_when_response_stays_within_tolerance() {
        let time = settling_time(&[1.4, 1.05, 1.01, 1.0], 1.0, 0.05, 0.5).unwrap();
        assert_eq!(time, Some(0.5));

        let no_settle = settling_time(&[1.4, 1.2, 1.1], 1.0, 0.05, 0.5).unwrap();
        assert_eq!(no_settle, None);
    }

    #[test]
    fn rejects_invalid_inputs() {
        assert_eq!(
            classify_gain(f64::NAN, 1.0),
            Err(StabilityError::InvalidGain)
        );
        assert_eq!(is_bounded(&[1.0], 0.0), Err(StabilityError::InvalidBound));
        assert_eq!(
            settling_time(&[1.0], 1.0, -0.1, 0.5),
            Err(StabilityError::InvalidTolerance)
        );
        assert_eq!(
            settling_time(&[1.0], 1.0, 0.1, 0.0),
            Err(StabilityError::InvalidTimestep)
        );
    }
}
