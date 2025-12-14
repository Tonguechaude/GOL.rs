//! # Color Plugin
//!
//! Plugin to manage colors in the game

use super::ColorConfig;
use bevy::prelude::*;

/// Plugin for managing colors
pub struct ColorPlugin;

impl Plugin for ColorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ColorConfig>()
            .add_systems(Startup, setup_initial_background_color)
            .add_systems(Update, update_clear_color_system);
    }
}

/// System that sets up the initial background color from the ColorConfig
fn setup_initial_background_color(
    color_config: Res<ColorConfig>,
    mut clear_color: ResMut<ClearColor>,
) {
    clear_color.0 = color_config.background_color;
}

/// System that updates the clear color (background) when the color configuration changes
fn update_clear_color_system(color_config: Res<ColorConfig>, mut clear_color: ResMut<ClearColor>) {
    if color_config.is_changed() {
        clear_color.0 = color_config.background_color;
    }
}
