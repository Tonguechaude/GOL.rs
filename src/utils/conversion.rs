//! # Conversion Module
//!
//! Utility functions for converting between different value ranges,
//! particularly for UI sliders and simulation parameters.

use crate::config::{MIN_PERIOD, MAX_PERIOD, DEFAULT_SCALE, MAX_SCALE};

/// Convert simulation period to slider value (1-100)
pub fn period_to_slider(period: f32) -> f32 {
    (100.0 - 99.0 * (period - MIN_PERIOD) / (MAX_PERIOD - MIN_PERIOD)).clamp(1.0, 100.0)
}

/// Convert slider value (1-100) to simulation period
pub fn slider_to_period(slider: f32) -> f32 {
    ((100.0 - slider) * (MAX_PERIOD - MIN_PERIOD) / 99.0 + MIN_PERIOD).clamp(MIN_PERIOD, MAX_PERIOD)
}

/// Convert camera scale to slider value (1-100)
pub fn scale_to_slider(scale: f32) -> f32 {
    (1.0 + 99.0 * (scale - DEFAULT_SCALE) / (MAX_SCALE - DEFAULT_SCALE)).clamp(1.0, 100.0)
}

/// Convert slider value (1-100) to camera scale
pub fn slider_to_scale(slider: f32) -> f32 {
    ((slider - 1.0) * (MAX_SCALE - DEFAULT_SCALE) / 99.0 + DEFAULT_SCALE)
        .clamp(DEFAULT_SCALE, MAX_SCALE)
}