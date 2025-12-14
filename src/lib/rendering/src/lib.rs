//! # Rendering Module
//!
//! Handles all visual rendering aspects of the Game of Life,
//! including cell sprites and grid display.

pub mod grid;
pub mod sprites;

pub use grid::*;
pub use sprites::*;

use bevy::prelude::{App, ClearColor, Plugin};
use gol_config::BG_COLOR;

/// Plugin for rendering systems
pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(BG_COLOR))
            .add_plugins(SpritePlugin)
            .add_plugins(GridPlugin);
    }
}
