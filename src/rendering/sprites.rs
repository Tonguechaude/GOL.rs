//! # Sprites Module
//!
//! Handles the visual representation of cells as sprites.

use bevy::prelude::*;
use crate::config::CELL_COLOR;
use crate::simulation::{CellPosition, Alive, CellSet};

/// Plugin for sprite rendering systems
pub struct SpritePlugin;

impl Plugin for SpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_new_cells_system.before(CellSet));
    }
}

/// System that adds visual components to newly spawned cells.
///
/// This system runs when cells are first created and adds the necessary
/// components to make them visible on screen.
pub fn draw_new_cells_system(
    mut commands: Commands,
    query: Query<(Entity, &CellPosition), (Added<CellPosition>, With<Alive>)>,
) {
    for (entity, pos) in query.iter() {
        commands.entity(entity)
            .insert(Sprite {
                color: CELL_COLOR,
                custom_size: Some(Vec2::new(1.0, 1.0)),
                ..Default::default()
            })
            .insert(Transform::from_xyz(pos.x as f32, pos.y as f32, 0.0));
    }
}