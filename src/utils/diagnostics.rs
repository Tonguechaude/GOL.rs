//! # Diagnostics Module
//!
//! FPS display and performance monitoring utilities.

use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};
use bevy::diagnostic::*;
use crate::config::FpsConfig;

/// Plugin for diagnostic systems
pub struct DiagnosticsPlugin;

impl Plugin for DiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin::default())
            .init_resource::<FpsConfig>()
            .add_systems(Update, toggle_fps_display)
            .add_systems(bevy_egui::EguiPrimaryContextPass, fps_display_system);
    }
}

/// Toggle FPS display with F3 key
pub fn toggle_fps_display(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut fps_config: ResMut<FpsConfig>,
) {
    if keyboard_input.just_pressed(KeyCode::F3) {
        fps_config.visible = !fps_config.visible;
    }
}

/// System to display FPS in an egui window
pub fn fps_display_system(
    mut contexts: EguiContexts,
    diagnostics: Res<DiagnosticsStore>,
    fps_config: Res<FpsConfig>,
) {
    if !fps_config.visible {
        return;
    }

    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };

    let fps_value = if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(value) = fps.smoothed() {
            format!("{:.2}", value)
        } else {
            "N/A".to_string()
        }
    } else {
        "N/A".to_string()
    };

    egui::Window::new("FPS")
        .resizable(false)
        .collapsible(false)
        .anchor(egui::Align2::RIGHT_TOP, egui::Vec2::new(-10.00, 10.0))
        .show(ctx, |ui| {
            ui.label(format!("FPS: {}", fps_value));

            // if let Some(frame_time) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FRAME_TIME) {
            //     if let Some(value) = frame_time.smoothed() {
            //         ui.label(format!("Frame Time: {:.2}ms", value));
            //     }
            // }
        });
}