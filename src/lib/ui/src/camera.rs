//! # Camera Module
//!
//! Camera initialization and movement controls for the 2D Game of Life view.

use bevy::prelude::{Commands, App, Startup, OrthographicProjection, Projection, Camera2d, Plugin};
use bevy::render::camera::ScalingMode;
use gol_config::DEFAULT_SCALE;

/// Plugin for camera-related systems
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_camera);
    }
}

/// Initializes the 2D camera for the Game of Life view.
///
/// Sets up an orthographic camera with a default scale that provides
/// a good overview of the simulation area.
pub fn init_camera(mut commands: Commands) {
    let projection = Projection::Orthographic(OrthographicProjection {
        scaling_mode: ScalingMode::WindowSize,
        scale: DEFAULT_SCALE,
        far: 1000.0,
        near: -1000.0,
        ..OrthographicProjection::default_2d()
    });
    commands.spawn((Camera2d, projection));
}
