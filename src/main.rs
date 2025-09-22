//! # Conway's Game of Life - Main Application
//!
//! This is the entry point for the Conway's Game of Life application.
//! It sets up a Bevy app with the necessary plugins for simulation and GUI.

use bevy::prelude::*;
use jeu_de_la_vie::{
    simulation::SimulationPlugin,
    rendering::RenderingPlugin,
    ui::UiPlugin,
    config::ConfigPlugin,
    utils::UtilsPlugin,
};

/// Entry point for the Conway's Game of Life application.
///
/// Creates a Bevy app with:
/// - Default Bevy plugins for rendering and input
/// - Custom window configuration suitable for web and desktop
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
        .add_plugins(ConfigPlugin)
        .add_plugins(SimulationPlugin)
        .add_plugins(RenderingPlugin)
        .add_plugins(UiPlugin)
        .add_plugins(UtilsPlugin)
        .run();
}