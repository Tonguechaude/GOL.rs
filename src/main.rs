//! # Conway's Game of Life - Main Application
//!
//! This is the entry point for the Conway's Game of Life application.
//! It sets up a Bevy app with the necessary plugins for simulation and GUI.

mod cellule;
mod gui;
mod info;

use bevy::prelude::*;
use cellule::CellSystem;
use gui::GuiSystem;
use bevy::diagnostic::*;
use info::FpsConfig;
use gui::CameraMovementConfig;

/// Entry point for the Conway's Game of Life application.
///
/// Creates a Bevy app with:
/// - Default Bevy plugins for rendering and input
/// - Custom window configuration suitable for web and desktop
/// - Game of Life simulation plugin (CellSystem)
/// - GUI plugin for user controls (GuiSystem)
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Conway's Game of Life".into(),
                fit_canvas_to_parent: true,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(CellSystem)
        .add_plugins(GuiSystem)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .init_resource::<FpsConfig>()
        .init_resource::<CameraMovementConfig>()
        .add_systems(Update, info::toggle_fps_display)
        .run();
}
