mod cellule;
mod gui;

use bevy::prelude::*;
use cellule::CelluleSystem;
use gui::GuiSystem;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Jeu de la Vie de Conway".into(),
                fit_canvas_to_parent: true,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(CelluleSystem)
        .add_plugins(GuiSystem)
        .run();
}
