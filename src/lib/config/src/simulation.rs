//! # Simulation Configuration
//!
//! Configuration parameters for the Game of Life simulation behavior.

use bevy::prelude::*;
use std::time::Duration;

/// Configuration parameters for the Game of Life simulation.
///
/// This resource controls the behavior of the simulation including
/// whether it's running automatically and at what speed.
#[derive(Resource, Debug)]
pub struct SimulationConfig {
    /// Whether the simulation is currently running automatically
    pub running: bool,
    /// Time delay between each generation update
    pub period: Duration,
    /// Flag to trigger a single step calculation when the simulation is paused
    pub calculate_next_gen: bool,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            running: true,
            period: Duration::from_secs(1),
            calculate_next_gen: false,
        }
    }
}
