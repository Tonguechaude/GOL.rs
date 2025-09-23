//! # Simulation Module
//!
//! This module contains all the core logic for Conway's Game of Life simulation.
//! It handles cell states, generation calculations, and simulation timing.

pub mod cell;
pub mod rules;
pub mod pattern;
pub mod generation;

pub use cell::*;
pub use rules::*;
pub use generation::*;

use bevy::prelude::*;

/// Bevy plugin that sets up the Game of Life simulation systems.
///
/// This plugin initializes all necessary resources and systems
/// for running Conway's Game of Life within a Bevy application.
pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CellPlugin)
            .add_plugins(GenerationPlugin);
    }
}