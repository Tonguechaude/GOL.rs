//! # Rendering Module
//!
//! Handles all visual rendering aspects of the Game of Life,
//! including cell sprites and grid display.

pub mod sprites;
pub mod grid;

pub use sprites::*;
pub use grid::*;

use bevy::prelude::*;
use crate::config::BG_COLOR;

/// Plugin for rendering systems
pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(BG_COLOR))
            .add_plugins(SpritePlugin)
            .add_plugins(GridPlugin);
    }
}