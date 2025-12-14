//! # Utils Module
//!
//! Utility functions and helper systems for the Game of Life application.

pub mod conversion;
pub mod diagnostics;

pub use conversion::*;
pub use diagnostics::*;

use bevy::prelude::{App, Plugin};

/// Plugin for utility systems
pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DiagnosticsPlugin);
    }
}
