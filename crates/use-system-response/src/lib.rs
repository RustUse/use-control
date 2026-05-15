#![forbid(unsafe_code)]
//! Simple system response helpers.
//!
//! The crate starts with a first-order step response model and a helper for
//! sampling the response at fixed time intervals.
//!
//! # Examples
//!
//! ```rust
//! use use_system_response::{first_order_response, sample_first_order_response};
//!
//! let initial = first_order_response(0.0, 1.0, 1.0, 0.0).unwrap();
//! let samples = sample_first_order_response(0.0, 1.0, 1.0, 0.5, 2).unwrap();
//!
//! assert_eq!(initial, 0.0);
//! assert_eq!(samples.len(), 3);
//! assert!(samples[1].value > samples[0].value);
//! ```

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StepResponsePoint {
    pub time: f64,
    pub value: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemResponseError {
    InvalidInput,
    InvalidTimeConstant,
    InvalidTime,
    InvalidTimestep,
    NonFiniteResponse,
}

pub fn first_order_response(
    initial: f64,
    target: f64,
    time_constant: f64,
    time: f64,
) -> Result<f64, SystemResponseError> {
    if !initial.is_finite() || !target.is_finite() {
        return Err(SystemResponseError::InvalidInput);
    }

    if !time_constant.is_finite() || time_constant <= 0.0 {
        return Err(SystemResponseError::InvalidTimeConstant);
    }

    if !time.is_finite() || time < 0.0 {
        return Err(SystemResponseError::InvalidTime);
    }

    let response = initial + (target - initial) * (1.0 - (-time / time_constant).exp());
    if !response.is_finite() {
        return Err(SystemResponseError::NonFiniteResponse);
    }

    Ok(response)
}

pub fn sample_first_order_response(
    initial: f64,
    target: f64,
    time_constant: f64,
    dt: f64,
    steps: usize,
) -> Result<Vec<StepResponsePoint>, SystemResponseError> {
    if !dt.is_finite() || dt <= 0.0 {
        return Err(SystemResponseError::InvalidTimestep);
    }

    let mut points = Vec::with_capacity(steps + 1);
    for step in 0..=steps {
        let time = step as f64 * dt;
        let value = first_order_response(initial, target, time_constant, time)?;
        points.push(StepResponsePoint { time, value });
    }

    Ok(points)
}

#[cfg(test)]
mod tests {
    use super::{SystemResponseError, first_order_response, sample_first_order_response};

    #[test]
    fn first_order_response_matches_expected_limits() {
        assert_eq!(first_order_response(0.0, 1.0, 1.0, 0.0).unwrap(), 0.0);

        let value = first_order_response(0.0, 1.0, 1.0, 1.0).unwrap();
        assert!((value - (1.0 - (-1.0f64).exp())).abs() < 1e-12);
    }

    #[test]
    fn samples_first_order_response_over_time() {
        let samples = sample_first_order_response(0.0, 1.0, 1.0, 0.5, 3).unwrap();

        assert_eq!(samples.len(), 4);
        assert_eq!(samples[0].time, 0.0);
        assert!(samples[1].value > samples[0].value);
        assert!(samples[3].value < 1.0);
    }

    #[test]
    fn rejects_invalid_inputs() {
        assert_eq!(
            first_order_response(f64::NAN, 1.0, 1.0, 0.0),
            Err(SystemResponseError::InvalidInput)
        );
        assert_eq!(
            first_order_response(0.0, 1.0, 0.0, 1.0),
            Err(SystemResponseError::InvalidTimeConstant)
        );
        assert_eq!(
            sample_first_order_response(0.0, 1.0, 1.0, 0.0, 3),
            Err(SystemResponseError::InvalidTimestep)
        );
    }
}
