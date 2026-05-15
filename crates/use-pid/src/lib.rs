#![forbid(unsafe_code)]
//! Minimal PID controller primitives.
//!
//! The controller keeps only the current gains and the state required for the
//! integral and derivative terms.
//!
//! # Examples
//!
//! ```rust
//! use use_pid::{PidController, PidGains};
//!
//! let mut controller = PidController::new(PidGains {
//!     kp: 2.0,
//!     ki: 0.0,
//!     kd: 0.0,
//! })
//! .unwrap();
//!
//! assert_eq!(controller.update(5.0, 3.0, 0.5).unwrap(), 4.0);
//! ```

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PidGains {
    pub kp: f64,
    pub ki: f64,
    pub kd: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PidState {
    pub previous_error: f64,
    pub integral: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PidController {
    pub gains: PidGains,
    pub state: PidState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PidError {
    InvalidGains,
    InvalidSignal,
    InvalidTimestep,
    NonFiniteOutput,
}

impl PidController {
    pub fn new(gains: PidGains) -> Result<Self, PidError> {
        if !gains.kp.is_finite() || !gains.ki.is_finite() || !gains.kd.is_finite() {
            return Err(PidError::InvalidGains);
        }

        Ok(Self {
            gains,
            state: PidState {
                previous_error: 0.0,
                integral: 0.0,
            },
        })
    }

    pub fn update(&mut self, setpoint: f64, measured: f64, dt: f64) -> Result<f64, PidError> {
        if !setpoint.is_finite() || !measured.is_finite() {
            return Err(PidError::InvalidSignal);
        }

        if !dt.is_finite() || dt <= 0.0 {
            return Err(PidError::InvalidTimestep);
        }

        let current_error = setpoint - measured;
        if !current_error.is_finite() {
            return Err(PidError::InvalidSignal);
        }

        self.state.integral += current_error * dt;
        let derivative = (current_error - self.state.previous_error) / dt;
        let output = self.gains.kp * current_error
            + self.gains.ki * self.state.integral
            + self.gains.kd * derivative;

        if !self.state.integral.is_finite() || !derivative.is_finite() || !output.is_finite() {
            return Err(PidError::NonFiniteOutput);
        }

        self.state.previous_error = current_error;
        Ok(output)
    }

    pub fn reset(&mut self) {
        self.state = PidState {
            previous_error: 0.0,
            integral: 0.0,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::{PidController, PidError, PidGains};

    #[test]
    fn proportional_only_behavior_stays_simple() {
        let mut controller = PidController::new(PidGains {
            kp: 2.0,
            ki: 0.0,
            kd: 0.0,
        })
        .unwrap();

        assert_eq!(controller.update(5.0, 3.0, 0.5).unwrap(), 4.0);
    }

    #[test]
    fn integral_term_accumulates_over_time() {
        let mut controller = PidController::new(PidGains {
            kp: 0.0,
            ki: 1.0,
            kd: 0.0,
        })
        .unwrap();

        assert_eq!(controller.update(2.0, 0.0, 0.5).unwrap(), 1.0);
        assert_eq!(controller.update(2.0, 0.0, 0.5).unwrap(), 2.0);
        assert_eq!(controller.state.integral, 2.0);
    }

    #[test]
    fn reset_clears_integral_and_previous_error() {
        let mut controller = PidController::new(PidGains {
            kp: 1.0,
            ki: 1.0,
            kd: 1.0,
        })
        .unwrap();
        controller.update(4.0, 1.0, 0.5).unwrap();

        controller.reset();

        assert_eq!(controller.state.previous_error, 0.0);
        assert_eq!(controller.state.integral, 0.0);
    }

    #[test]
    fn rejects_invalid_inputs() {
        assert_eq!(
            PidController::new(PidGains {
                kp: f64::NAN,
                ki: 0.0,
                kd: 0.0,
            }),
            Err(PidError::InvalidGains)
        );

        let mut controller = PidController::new(PidGains {
            kp: 1.0,
            ki: 0.0,
            kd: 0.0,
        })
        .unwrap();

        assert_eq!(
            controller.update(1.0, 0.0, 0.0),
            Err(PidError::InvalidTimestep)
        );
        assert_eq!(
            controller.update(f64::NAN, 0.0, 1.0),
            Err(PidError::InvalidSignal)
        );
    }
}
