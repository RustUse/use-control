#![forbid(unsafe_code)]
//! Thin facade for the `use-control` workspace.
//!
//! The crate reexports the focused control-system crates directly so consumers
//! can opt into one dependency while still using the smaller APIs.
//!
//! # Examples
//!
//! ```rust
//! use use_control::*;
//!
//! let mut controller = PidController::new(PidGains {
//!     kp: 2.0,
//!     ki: 0.0,
//!     kd: 0.0,
//! })
//! .unwrap();
//! let output = controller.update(5.0, 3.0, 0.5).unwrap();
//! let signal = ControlSignal::new(output).unwrap().clamp(0.0, 10.0).unwrap();
//!
//! assert_eq!(signal.value(), 4.0);
//! assert_eq!(classify_gain(0.8, 1.0).unwrap(), Stability::Stable);
//! ```

pub use use_control_error;
pub use use_control_error::*;
pub use use_control_signal;
pub use use_control_signal::*;
pub use use_feedback;
pub use use_feedback::*;
pub use use_pid;
pub use use_pid::*;
pub use use_setpoint;
pub use use_setpoint::*;
pub use use_stability;
pub use use_stability::*;
pub use use_system_response;
pub use use_system_response::*;

#[cfg(test)]
mod tests {
    use super::{
        ControlSignal, FeedbackDirection, FeedbackLoop, PidController, PidGains, Setpoint,
        Stability, StepResponsePoint, classify_gain, deadband, first_order_response, saturate,
    };

    #[test]
    fn facade_reexports_workspace_apis() {
        let loop_gain = FeedbackLoop::new(0.5, FeedbackDirection::Negative).unwrap();
        assert_eq!(loop_gain.apply(10.0, 2.0), 9.0);

        let mut controller = PidController::new(PidGains {
            kp: 2.0,
            ki: 0.0,
            kd: 0.0,
        })
        .unwrap();
        assert_eq!(controller.update(5.0, 3.0, 0.5).unwrap(), 4.0);

        let setpoint = Setpoint::new(10.0, 0.2).unwrap();
        assert!(setpoint.is_reached(9.9));

        assert_eq!(deadband(0.05, 0.1).unwrap(), 0.0);
        assert_eq!(
            ControlSignal::new(12.0)
                .unwrap()
                .clamp(0.0, 10.0)
                .unwrap()
                .value(),
            10.0
        );
        assert_eq!(saturate(-7.0, 5.0).unwrap(), -5.0);

        let response = first_order_response(0.0, 1.0, 1.0, 1.0).unwrap();
        assert!(response > 0.0);
        assert_eq!(classify_gain(0.8, 1.0).unwrap(), Stability::Stable);

        let point = StepResponsePoint {
            time: 0.5,
            value: response,
        };
        assert_eq!(point.time, 0.5);
    }
}
