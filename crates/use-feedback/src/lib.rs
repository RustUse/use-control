#![forbid(unsafe_code)]
//! Primitive feedback loop helpers.
//!
//! The crate intentionally keeps feedback logic explicit: a gain plus a simple
//! positive or negative direction.
//!
//! # Examples
//!
//! ```rust
//! use use_feedback::{negative_feedback, FeedbackDirection, FeedbackLoop};
//!
//! let loop_gain = FeedbackLoop::new(0.5, FeedbackDirection::Negative).unwrap();
//! assert_eq!(loop_gain.apply(10.0, 2.0), 9.0);
//! assert_eq!(negative_feedback(10.0, 2.0, 0.5).unwrap(), 9.0);
//! ```

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeedbackDirection {
    Positive,
    Negative,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FeedbackLoop {
    pub gain: f64,
    pub direction: FeedbackDirection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeedbackError {
    InvalidGain,
}

impl FeedbackLoop {
    pub fn new(gain: f64, direction: FeedbackDirection) -> Result<Self, FeedbackError> {
        if !gain.is_finite() {
            return Err(FeedbackError::InvalidGain);
        }

        Ok(Self { gain, direction })
    }

    pub fn apply(&self, input: f64, feedback: f64) -> f64 {
        match self.direction {
            FeedbackDirection::Positive => input + self.gain * feedback,
            FeedbackDirection::Negative => input - self.gain * feedback,
        }
    }
}

pub fn negative_feedback(input: f64, feedback: f64, gain: f64) -> Result<f64, FeedbackError> {
    Ok(FeedbackLoop::new(gain, FeedbackDirection::Negative)?.apply(input, feedback))
}

pub fn positive_feedback(input: f64, feedback: f64, gain: f64) -> Result<f64, FeedbackError> {
    Ok(FeedbackLoop::new(gain, FeedbackDirection::Positive)?.apply(input, feedback))
}

#[cfg(test)]
mod tests {
    use super::{
        negative_feedback, positive_feedback, FeedbackDirection, FeedbackError, FeedbackLoop,
    };

    #[test]
    fn applies_negative_feedback() {
        let loop_gain = FeedbackLoop::new(0.5, FeedbackDirection::Negative).unwrap();

        assert_eq!(loop_gain.apply(10.0, 2.0), 9.0);
        assert_eq!(negative_feedback(10.0, 2.0, 0.5).unwrap(), 9.0);
    }

    #[test]
    fn applies_positive_feedback() {
        let loop_gain = FeedbackLoop::new(0.5, FeedbackDirection::Positive).unwrap();

        assert_eq!(loop_gain.apply(10.0, 2.0), 11.0);
        assert_eq!(positive_feedback(10.0, 2.0, 0.5).unwrap(), 11.0);
    }

    #[test]
    fn rejects_non_finite_gain() {
        assert_eq!(
            FeedbackLoop::new(f64::NAN, FeedbackDirection::Negative),
            Err(FeedbackError::InvalidGain)
        );
        assert_eq!(
            positive_feedback(10.0, 2.0, f64::INFINITY),
            Err(FeedbackError::InvalidGain)
        );
    }
}
