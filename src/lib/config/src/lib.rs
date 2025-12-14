//! # Configuration Module
//!
//! Contains all configuration and parameter structures for the Game of Life application.

pub mod color;
pub mod constants;
pub mod display;
pub mod simulation;

pub use color::*;
pub use constants::*;
pub use display::*;
pub use simulation::*;

use bevy::prelude::*;

/// Plugin for configuration resources
pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SimulationConfig>()
            .init_resource::<DisplayConfig>()
            .init_resource::<CameraConfig>();
    }
}
