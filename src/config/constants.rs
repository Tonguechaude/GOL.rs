//! # Constants
//!
//! Global constants used throughout the application.

use bevy::prelude::*;

/// Type alias for time values in seconds
pub type Seconds = f32;

/// Background color for the simulation window
pub const BG_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
/// Color used to render living cells
pub const CELL_COLOR: Color = Color::srgb(0.0, 0.0, 0.0);

/// Default camera scale (zoomed out view)
pub const DEFAULT_SCALE: f32 = 1.0 / 40.0;
/// Maximum camera scale (zoomed in view)
pub const MAX_SCALE: f32 = 1.0;

/// Minimum time period between generations (fastest speed)
pub const MIN_PERIOD: Seconds = 0.01;
/// Maximum time period between generations (slowest speed)
pub const MAX_PERIOD: Seconds = 1.5;

/// Zoom step factor for keyboard zoom controls
pub const ZOOM_STEP: f32 = 0.1;

/// Base movement speed for camera
pub const BASE_SPEED: f32 = 25.0;
/// Maximum movement speed for camera in turbo mode
pub const MAX_SPEED: f32 = 125.0;