//! # Utils Module
//!
//! Utility functions and helper systems for the Game of Life application.

pub mod diagnostics;
pub mod conversion;

pub use diagnostics::*;
pub use conversion::*;

use bevy::prelude::*;

/// Plugin for utility systems
pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DiagnosticsPlugin);
    }
}