//! # UI Module
//!
//! User interface components and interaction handling for the Game of Life application.

pub mod camera;
pub mod controls;
pub mod input;
pub mod modals;
pub mod pattern;

pub use camera::*;
pub use controls::*;
pub use input::*;
pub use modals::*;
pub use pattern::*;

use bevy::prelude::{Plugin, App};
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
