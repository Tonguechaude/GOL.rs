//! # Grid Module
//!
//! Handles the visual rendering of the grid overlay.

use bevy::prelude::{App, Camera, GlobalTransform, Plugin, Projection, Query, Res, Vec2, Vec3};
use bevy_egui::{
    EguiContexts,
    egui::{self, Color32},
};
use gol_config::{DEFAULT_SCALE, DisplayConfig, MAX_SCALE};

/// Plugin for grid rendering systems
pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(bevy_egui::EguiPrimaryContextPass, draw_grid_system);
    }
}

/// System that renders the grid overlay using egui
pub fn draw_grid_system(
    mut contexts: EguiContexts,
    display_config: Res<DisplayConfig>,
    q_camera: Query<(&Camera, &Projection, &GlobalTransform)>,
) {
    if !display_config.grid_visible {
        return;
    }

    // Use semi-transparent color for rows in the grid
    const LINE_COLOR: Color32 = Color32::from_gray(128);
    let (camera, camera_projection, camera_transform) = match q_camera.single() {
        Ok(data) => data,
        Err(_) => return,
    };

    let camera_scale = match camera_projection {
        Projection::Orthographic(orthographic) => orthographic.scale,
        _ => return,
    };

    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };

    let transparent_frame = egui::containers::Frame {
        fill: Color32::TRANSPARENT,
        ..Default::default()
    };
    let line_width = (1.0 - (camera_scale - DEFAULT_SCALE) / (MAX_SCALE - DEFAULT_SCALE)).powi(10);

    egui::CentralPanel::default()
        .frame(transparent_frame)
        .show(ctx, |ui| {
            let (response, painter) = ui.allocate_painter(
                bevy_egui::egui::Vec2::new(ui.available_width(), ui.available_height()),
                egui::Sense::hover(),
            );
            let Ok(ray_top_left) =
                camera.viewport_to_world(camera_transform, Vec2 { x: 0.0, y: 0.0 })
            else {
                return;
            };
            let visible_top_left = ray_top_left.origin.truncate();
            let (x_min, y_max) = (
                visible_top_left.x.round() as isize,
                visible_top_left.y.round() as isize,
            );
            let Ok(ray_bottom_right) = camera.viewport_to_world(
                camera_transform,
                Vec2 {
                    x: response.rect.right(),
                    y: response.rect.bottom(),
                },
            ) else {
                return;
            };
            let visible_bottom_right = ray_bottom_right.origin.truncate();
            let (x_max, y_min) = (
                visible_bottom_right.x.round() as isize,
                visible_bottom_right.y.round() as isize,
            );

            // Draw vertical lines
            for x in x_min..=x_max {
                let Ok(start) = camera.world_to_viewport(
                    camera_transform,
                    Vec3 {
                        x: x as f32 - 0.5,
                        y: y_min as f32 - 0.5,
                        z: 0.0,
                    },
                ) else {
                    continue;
                };
                let start_pos = egui::Pos2::new(start.x, start.y);
                let Ok(end) = camera.world_to_viewport(
                    camera_transform,
                    Vec3 {
                        x: x as f32 - 0.5,
                        y: y_max as f32 + 0.5,
                        z: 0.0,
                    },
                ) else {
                    continue;
                };
                let end_pos = egui::Pos2::new(end.x, end.y);
                painter.add(egui::Shape::LineSegment {
                    points: [start_pos, end_pos],
                    stroke: egui::Stroke {
                        width: line_width,
                        color: LINE_COLOR,
                    }
                    .into(),
                });
            }

            // Draw horizontal lines
            for y in y_min..=y_max {
                let Ok(start) = camera.world_to_viewport(
                    camera_transform,
                    Vec3 {
                        x: x_min as f32 - 0.5,
                        y: y as f32 - 0.5,
                        z: 0.0,
                    },
                ) else {
                    continue;
                };
                let start_pos = egui::Pos2::new(start.x, start.y);
                let Ok(end) = camera.world_to_viewport(
                    camera_transform,
                    Vec3 {
                        x: x_max as f32 + 0.5,
                        y: y as f32 - 0.5,
                        z: 0.0,
                    },
                ) else {
                    continue;
                };
                let end_pos = egui::Pos2::new(end.x, end.y);
                painter.add(egui::Shape::LineSegment {
                    points: [start_pos, end_pos],
                    stroke: egui::Stroke {
                        width: line_width,
                        color: LINE_COLOR,
                    }
                    .into(),
                });
            }
        });
}
