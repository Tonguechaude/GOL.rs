use bevy::prelude::*;

#[derive(Resource)]
pub struct FpsConfig {
    pub visible: bool,
}

impl Default for FpsConfig {
    fn default() -> Self {
        Self { visible: false }
    }
}

// toggle fps display with F3
pub fn toggle_fps_display(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut fps_config: ResMut<FpsConfig>,
) {
    if keyboard_input.just_pressed(KeyCode::F3) {
        fps_config.visible = !fps_config.visible;
    }
}