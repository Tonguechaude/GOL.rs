//! # Sprites Module
//!
//! Handles the visual representation of cells as sprites.

use bevy::prelude::*;
use gol_config::ColorConfig;
use gol_simulation::{Alive, CellPosition, CellSet};

/// Plugin for sprite rendering systems
pub struct SpritePlugin;

impl Plugin for SpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                draw_new_cells_system.before(CellSet),
                update_cell_colors_system,
            ),
        );
    }
}

/// System that adds visual components to newly spawned cells.
///
/// This system runs when cells are first created and adds the necessary
/// components to make them visible on screen.
pub fn draw_new_cells_system(
    mut commands: Commands,
    color_config: Res<ColorConfig>,
    query: Query<(Entity, &CellPosition), (With<Alive>, Without<Sprite>)>,
) {
    for (entity, pos) in query.iter() {
        commands
            .entity(entity)
            .insert(Sprite {
                color: color_config.cell_color,
                custom_size: Some(Vec2::new(1.0, 1.0)),
                ..Default::default()
            })
            .insert(Transform::from_xyz(pos.x as f32, pos.y as f32, 0.0));
    }
}

/// System that updates the colors of existing cells when the color configuration changes
pub fn update_cell_colors_system(
    color_config: Res<ColorConfig>,
    mut query: Query<&mut Sprite, (With<CellPosition>, With<Alive>)>,
) {
    // Verify and correct the cell color every frame
    for mut sprite in query.iter_mut() {
        if sprite.color != color_config.cell_color {
            sprite.color = color_config.cell_color;
        }
    }
}
