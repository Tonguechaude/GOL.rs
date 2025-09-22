//! # UI Module
//!
//! User interface components and interaction handling for the Game of Life application.

pub mod controls;
pub mod modals;
pub mod camera;
pub mod input;

pub use controls::*;
pub use modals::*;
pub use camera::*;
pub use input::*;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;

/// Bevy plugin that sets up the GUI systems and resources.
///
/// This plugin adds all the necessary systems for rendering the interface,
/// handling user input, and managing the Game of Life grid interaction.
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin::default())
            .add_plugins(CameraPlugin)
            .add_plugins(InputPlugin)
            .add_plugins(ControlsPlugin)
            .add_plugins(ModalsPlugin);
    }
}