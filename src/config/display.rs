//! # Display Configuration
//!
//! Configuration parameters for visual display and camera behavior.

use bevy::prelude::*;

/// GUI-specific configuration parameters.
///
/// Contains settings for the user interface that don't directly
/// affect the simulation logic but control display options.
#[derive(Resource, Debug)]
pub struct DisplayConfig {
    /// Width of the grid for random cell generation
    pub random_grid_width: u16,
    /// Whether to display the grid overlay
    pub grid_visible: bool,
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self { 
            random_grid_width: 50u16, 
            grid_visible: true 
        }
    }
}

/// Camera movement and control configuration
#[derive(Resource, Default)]
pub struct CameraConfig {
    /// Whether turbo mode (faster movement) is enabled
    pub turbo_mode: bool,
}

/// FPS display configuration
#[derive(Resource)]
pub struct FpsConfig {
    /// Whether FPS counter is visible
    pub visible: bool,
}

impl Default for FpsConfig {
    fn default() -> Self {
        Self { visible: false }
    }
}