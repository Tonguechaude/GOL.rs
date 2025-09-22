//! # Controls Module
//!
//! Main control panel for the Game of Life simulation.

use std::time::Duration;
use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};
use crate::config::{SimulationConfig, DisplayConfig};
use crate::simulation::{Alive, DeadCellPool};
use crate::utils::{period_to_slider, slider_to_period, scale_to_slider, slider_to_scale};

/// Plugin for control panel systems
pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(bevy_egui::EguiPrimaryContextPass, control_panel_system);
    }
}

/// Main control panel system that renders the GUI controls
pub fn control_panel_system(
    mut commands: Commands,
    mut contexts: EguiContexts,
    mut simulation_config: ResMut<SimulationConfig>,
    mut display_config: ResMut<DisplayConfig>,
    mut q_camera: Query<(&mut Projection, &GlobalTransform)>,
    q_cells: Query<Entity, With<Alive>>,
    mut dead_pool: ResMut<DeadCellPool>,
) {
    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };
    ctx.set_visuals(egui::style::Visuals::light());
    
    let Ok((mut camera_projection, camera_transform)) = q_camera.single_mut() else {
        eprintln!("Erreur camera: impossible d'obtenir une seule caméra");
        return;
    };

    let (speed_slider_init, scale_slider_init, mut scale_slider_val) = 
        match camera_projection.as_mut() {
            Projection::Orthographic(orthographic) => {
                let speed_slider = period_to_slider(simulation_config.period.as_secs_f32());
                let scale_slider = scale_to_slider(orthographic.scale);
                (speed_slider, scale_slider, scale_slider)
            },
            _ => return,
        };
    
    let mut speed_slider = speed_slider_init;
    let separator = |ui: &mut egui::Ui| ui.add(egui::Separator::default());

    egui::Window::new("Game of Life").resizable(false).show(ctx, |ui| {
        ui.horizontal(|ui| {
            if ui.button("Clear Grid").clicked() {
                simulation_config.running = false;
                clear_cells(&mut commands, &q_cells, &mut dead_pool);
            }
        });
        
        ui.horizontal(|ui| {
            ui.add(egui::DragValue::new(&mut display_config.random_grid_width).suffix(" width"));
            if ui.button("Random Cells").clicked() {
                let offset = -(display_config.random_grid_width as isize) / 2;
                let width = display_config.random_grid_width as usize;
                clear_cells(&mut commands, &q_cells, &mut dead_pool);
                generate_random_cells(&mut commands, offset, offset, width, width);
            }
        });
        
        separator(ui);
        ui.vertical(|ui| {
            ui.add(
                egui::Slider::new(&mut speed_slider, 1.0..=100.0).text("Speed").show_value(false),
            );
            ui.add(
                egui::Slider::new(&mut scale_slider_val, 1.0..=100.0)
                    .text("Camera Distance")
                    .show_value(false)
                    .logarithmic(true),
            );
        });
        
        separator(ui);
        ui.horizontal(|ui| {
            let play_text = if simulation_config.running { "Pause" } else { "Start" };
            if ui.button(play_text).clicked() {
                simulation_config.running = !simulation_config.running;
            }
            let next_step_btn =
                ui.add_enabled(!simulation_config.running, egui::Button::new("Next Generation"));
            if !simulation_config.running && next_step_btn.clicked() {
                simulation_config.calculate_next_gen = true;
            };
        });
        
        separator(ui);
        ui.vertical(|ui| {
            ui.checkbox(&mut display_config.grid_visible, "Show Grid");
        });
        
        separator(ui);
        ui.vertical(|ui| {
            let x = camera_transform.translation().x;
            let y = camera_transform.translation().y;
            ui.label(format!("Current Position: x: {x}, y: {y}"));
            ui.add_space(5.);
            ui.label("Click on the grid when simulation is paused!");
            ui.label("Use arrow keys to move the camera!");
        });
    });

    // Apply camera scale changes
    if let Projection::Orthographic(orthographic) = camera_projection.as_mut() {
        if scale_slider_init != scale_slider_val {
            orthographic.scale = slider_to_scale(scale_slider_val);
        }
    }

    // Apply speed changes
    if speed_slider_init != speed_slider {
        simulation_config.period = Duration::from_secs_f32(slider_to_period(speed_slider));
    }
}

/// Removes all living cells from the simulation
fn clear_cells(
    commands: &mut Commands, 
    q_cells: &Query<Entity, With<Alive>>,
    dead_pool: &mut ResMut<DeadCellPool>
) {
    for entity in q_cells.iter() {
        commands.entity(entity)
            .remove::<Alive>()
            .insert(Visibility::Hidden);
        dead_pool.entities.push(entity);
    }
}

/// Generates random cells in a rectangular area
fn generate_random_cells(commands: &mut Commands, x: isize, y: isize, width: usize, height: usize) {
    use rand::Rng;
    use crate::simulation::CellPosition;
    use crate::config::CELL_COLOR;
    
    let mut rng = rand::rng();
    for coord_x in x..(x + width as isize) {
        for coord_y in y..(y + height as isize) {
            if rng.random_range(0..10) > 7 {
                commands.spawn((
                    CellPosition { x: coord_x, y: coord_y },
                    Alive,
                    Sprite {
                        color: CELL_COLOR,
                        custom_size: Some(Vec2::new(1.0, 1.0)),
                        ..Default::default()
                    },
                    Transform::from_xyz(coord_x as f32, coord_y as f32, 0.0),
                ));
            }
        }
    }
}