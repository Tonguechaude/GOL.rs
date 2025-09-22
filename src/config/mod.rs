//! # Configuration Module
//!
//! Contains all configuration and parameter structures for the Game of Life application.

pub mod simulation;
pub mod display;
pub mod constants;

pub use simulation::*;
pub use display::*;
pub use constants::*;

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