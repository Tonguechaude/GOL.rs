//! # GUI Module
//!
//! This module provides the graphical user interface for the Game of Life simulation.
//! It includes controls for starting/stopping the simulation, adjusting speed,
//! camera movement, and interactive cell placement.

use std::time::Duration;

use crate::cellule::{CellParams, CellPosition, CellSet};
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{
    EguiContexts, EguiPlugin,
    egui::{self, Color32, Ui},
};
use egui_modal::Modal;
use rand::Rng;

/// Type alias for time values in seconds
type Seconds = f32;

/// Background color for the simulation window
const BG_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
/// Color used to render living cells
const CELL_COLOR: Color = Color::srgb(0.0, 0.0, 0.0);

/// Default camera scale (zoomed out view)
const DEFAULT_SCALE: f32 = 1.0 / 40.0;
/// Maximum camera scale (zoomed in view)
const MAX_SCALE: f32 = 1.0;

/// Minimum time period between generations (fastest speed)
const MIN_PERIOD: Seconds = 0.01;
/// Maximum time period between generations (slowest speed)
const MAX_PERIOD: Seconds = 1.5;

/// Bevy plugin that sets up the GUI systems and resources.
///
/// This plugin adds all the necessary systems for rendering the interface,
/// handling user input, and drawing the Game of Life grid.
pub struct GuiSystem;

impl Plugin for GuiSystem {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(BG_COLOR))
            .insert_resource(GuiParams::default())
            .add_plugins(EguiPlugin)
            .add_systems(Startup, init_camera)
            .add_systems(Update, gui_system)
            .add_systems(Update, mouse_click_system)
            .add_systems(Update, keyboard_input_system)
            .add_systems(Update, draw_new_cells_system.before(CellSet))
            .add_systems(
                Update,
                draw_grid_system
                    .after(draw_new_cells_system)
                    .run_if(|gui_params: Res<GuiParams>| gui_params.grid_visible),
            );
    }
}

/// GUI-specific configuration parameters.
///
/// Contains settings for the user interface that don't directly
/// affect the simulation logic but control display options.
#[derive(Resource, Debug)]
pub struct GuiParams {
    /// Width of the grid for random cell generation
    pub random_grid_width: u16,
    /// Whether to display the grid overlay
    pub grid_visible: bool,
}

impl Default for GuiParams {
    fn default() -> Self {
        Self { random_grid_width: 50u16, grid_visible: true }
    }
}

/// Initializes the 2D camera for the Game of Life view.
///
/// Sets up an orthographic camera with a default scale that provides
/// a good overview of the simulation area.
fn init_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            scale: DEFAULT_SCALE,
            far: 1000.0,
            near: -1000.0,
            ..OrthographicProjection::default_2d()
        },
    ));
}

/// Main GUI system that renders the control panel.
///
/// Creates the main control window with buttons for:
/// - Starting/stopping the simulation
/// - Clearing the grid
/// - Generating random patterns
/// - Adjusting simulation speed and camera zoom
/// - Toggling grid display
fn gui_system(
    mut commands: Commands,
    mut contexts: EguiContexts,
    mut cell_params: ResMut<CellParams>,
    mut gui_params: ResMut<GuiParams>,
    mut q_camera: Query<(&mut OrthographicProjection, &GlobalTransform)>,
    q_cells: Query<Entity, With<CellPosition>>,
) {
    let ctx = contexts.ctx_mut();
    ctx.set_visuals(egui::style::Visuals::light());
    let (mut camera_proj, camera_transform) = match q_camera.get_single_mut() {
        Ok(data) => data,
        Err(_) => return,
    };
    let speed_slider_init = period_to_slider(cell_params.period.as_secs_f32());
    let mut speed_slider = speed_slider_init;
    let scale_slider_init = scale_to_slider(camera_proj.scale);
    let mut scale_slider_val = scale_slider_init;

    let reset_modal = {
        let modal = Modal::new(ctx, "reset_modal");
        modal.show(|ui| {
            modal.title(ui, "Kill all cells!");
            modal.frame(ui, |ui| {
                modal.body(ui, "Kill all cells?");
            });
            modal.buttons(ui, |ui| {
                modal.button(ui, "No");
                if modal.button(ui, "Yes").clicked() {
                    cell_params.running = false;
                    clear_cells(&mut commands, &q_cells);
                };
            });
        });
        modal
    };
    let random_modal = {
        let modal = Modal::new(ctx, "random_modal");
        modal.show(|ui| {
            modal.title(ui, "Random Generation");
            modal.frame(ui, |ui| {
                modal.body(ui, "Fill grid randomly?");
            });
            modal.buttons(ui, |ui| {
                modal.button(ui, "No");
                if modal.button(ui, "Yes").clicked() {
                    let offset = -(gui_params.random_grid_width as isize) / 2;
                    let width = gui_params.random_grid_width as usize;
                    clear_cells(&mut commands, &q_cells);
                    generate_random_cells(&mut commands, offset, offset, width, width);
                };
            });
        });
        modal
    };
    let separator = |ui: &mut Ui| ui.add(egui::Separator::default());

    egui::Window::new("Game of Life").resizable(false).show(ctx, |ui| {
        ui.horizontal(|ui| {
            if ui.button("Clear Grid").clicked() {
                reset_modal.open();
            }
        });
        ui.horizontal(|ui| {
            ui.add(egui::DragValue::new(&mut gui_params.random_grid_width).suffix(" width"));
            if ui.button("Random Cells").clicked() {
                random_modal.open();
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
            let play_text = if cell_params.running { "Pause" } else { "Start" };
            if ui.button(play_text).clicked() {
                cell_params.running = !cell_params.running;
            }
            let next_step_btn =
                ui.add_enabled(!cell_params.running, egui::Button::new("Next Generation"));
            if !cell_params.running && next_step_btn.clicked() {
                cell_params.calculate_next_gen = true;
            };
        });
        separator(ui);
        ui.vertical(|ui| {
            ui.checkbox(&mut gui_params.grid_visible, "Show Grid");
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

    if scale_slider_init != scale_slider_val {
        camera_proj.scale = slider_to_scale(scale_slider_val);
    }
    if speed_slider_init != speed_slider {
        cell_params.period = Duration::from_secs_f32(slider_to_period(speed_slider));
    }
}

/// System that adds visual components to newly spawned cells.
///
/// This system runs when cells are first created and adds the necessary
/// Sprite and Transform components to make them visible on screen.
fn draw_new_cells_system(
    mut commands: Commands,
    query: Query<(Entity, &CellPosition), Added<CellPosition>>,
) {
    for (entity, pos) in query.iter() {
        commands.entity(entity).insert(Sprite {
            color: CELL_COLOR,
            custom_size: Some(Vec2::new(1.0, 1.0)),
            ..Default::default()
        });

        commands.entity(entity).insert(Transform::from_xyz(pos.x as f32, pos.y as f32, 0.0));
    }
}

/// Handles mouse clicks to toggle cells on/off.
///
/// When the simulation is paused and the user clicks on the grid,
/// this system will either create a new cell or remove an existing one
/// at the clicked position.
fn mouse_click_system(
    mut commands: Commands,
    cell_params: Res<CellParams>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    q_cellpos: Query<(Entity, &CellPosition)>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    if cell_params.running || !buttons.just_released(MouseButton::Left) {
        return;
    }
    let Ok(window) = q_windows.get_single() else {
        return;
    };
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };
    let Ok((camera, camera_transform)) = q_camera.get_single() else {
        return;
    };

    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };
    let position_cible = ray.origin.truncate().round();

    debug!("Click position: {position_cible}");
    let new_cell = CellPosition { x: position_cible.x as isize, y: position_cible.y as isize };
    for (entity, cell_position) in q_cellpos.iter() {
        if cell_position == &new_cell {
            commands.entity(entity).despawn();
            return;
        }
    }
    commands.spawn(new_cell);
}

/// Handles keyboard input for camera movement.
///
/// Arrow keys move the camera around the Game of Life grid,
/// allowing users to explore different areas of the simulation.
fn keyboard_input_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut q_camera_transform: Query<&mut Transform, With<Camera>>,
) {
    let (mut x, mut y) = (0, 0);
    if keys.pressed(KeyCode::ArrowLeft) {
        x += -1;
    }
    if keys.pressed(KeyCode::ArrowRight) {
        x += 1;
    }
    if keys.pressed(KeyCode::ArrowUp) {
        y += 1;
    }
    if keys.pressed(KeyCode::ArrowDown) {
        y += -1;
    }
    let Ok(mut transform) = q_camera_transform.get_single_mut() else {
        return;
    };
    transform.translation += Vec3::new(x as f32, y as f32, 0.0);
}

fn draw_grid_system(
    mut contexts: EguiContexts,
    q_camera: Query<(&Camera, &OrthographicProjection, &GlobalTransform)>,
) {
    const LINE_COLOR: Color32 = Color32::BLACK;
    let (camera, camera_proj, camera_transform) = match q_camera.get_single() {
        Ok(data) => data,
        Err(_) => return,
    };
    let ctx = contexts.ctx_mut();
    let transparent_frame =
        egui::containers::Frame { fill: Color32::TRANSPARENT, ..Default::default() };
    let line_width =
        (1.0 - (camera_proj.scale - DEFAULT_SCALE) / (MAX_SCALE - DEFAULT_SCALE)).powi(10);

    egui::CentralPanel::default().frame(transparent_frame).show(ctx, |ui| {
        let (response, painter) = ui.allocate_painter(
            bevy_egui::egui::Vec2::new(ui.available_width(), ui.available_height()),
            egui::Sense { click: false, drag: false, focusable: false },
        );
        let Ok(ray_top_left) = camera.viewport_to_world(camera_transform, Vec2 { x: 0.0, y: 0.0 })
        else {
            return;
        };
        let visible_top_left = ray_top_left.origin.truncate();
        let (x_min, y_max) =
            (visible_top_left.x.round() as isize, visible_top_left.y.round() as isize);
        let Ok(ray_bottom_right) = camera.viewport_to_world(
            camera_transform,
            Vec2 { x: response.rect.right(), y: response.rect.bottom() },
        ) else {
            return;
        };
        let visible_bottom_right = ray_bottom_right.origin.truncate();
        let (x_max, y_min) =
            (visible_bottom_right.x.round() as isize, visible_bottom_right.y.round() as isize);
        for x in x_min..=x_max {
            let Ok(start) = camera.world_to_viewport(
                camera_transform,
                Vec3 { x: x as f32 - 0.5, y: y_min as f32 - 0.5, z: 0.0 },
            ) else {
                continue;
            };
            let start_pos = egui::Pos2::new(start.x, start.y);
            let Ok(end) = camera.world_to_viewport(
                camera_transform,
                Vec3 { x: x as f32 - 0.5, y: y_max as f32 + 0.5, z: 0.0 },
            ) else {
                continue;
            };
            let end_pos = egui::Pos2::new(end.x, end.y);
            painter.add(egui::Shape::LineSegment {
                points: [start_pos, end_pos],
                stroke: egui::Stroke { width: line_width, color: LINE_COLOR }.into(),
            });
        }
        for y in y_min..=y_max {
            let Ok(start) = camera.world_to_viewport(
                camera_transform,
                Vec3 { x: x_min as f32 - 0.5, y: y as f32 - 0.5, z: 0.0 },
            ) else {
                continue;
            };
            let start_pos = egui::Pos2::new(start.x, start.y);
            let Ok(end) = camera.world_to_viewport(
                camera_transform,
                Vec3 { x: x_max as f32 + 0.5, y: y as f32 - 0.5, z: 0.0 },
            ) else {
                continue;
            };
            let end_pos = egui::Pos2::new(end.x, end.y);
            painter.add(egui::Shape::LineSegment {
                points: [start_pos, end_pos],
                stroke: egui::Stroke { width: line_width, color: LINE_COLOR }.into(),
            });
        }
    });
}

/// Removes all living cells from the simulation.
///
/// Used by the "clear grid" functionality to reset the simulation
/// to an empty state.
fn clear_cells(commands: &mut Commands, q_cells: &Query<Entity, With<CellPosition>>) {
    let cells_to_remove: Vec<Entity> = q_cells.iter().collect();
    for entity in cells_to_remove {
        commands.entity(entity).despawn();
    }
}

/// Generates a random pattern of cells in a rectangular area.
///
/// Creates living cells randomly within the specified bounds.
/// Each position has a 50% chance of containing a living cell.
///
/// # Arguments
/// * `x` - Starting x-coordinate of the generation area
/// * `y` - Starting y-coordinate of the generation area
/// * `width` - Width of the generation area
/// * `height` - Height of the generation area
fn generate_random_cells(commands: &mut Commands, x: isize, y: isize, width: usize, height: usize) {
    let mut rng = rand::rng();
    for coord_x in x..(x + width as isize) {
        for coord_y in y..(y + height as isize) {
            if rng.random::<bool>() {
                commands.spawn(CellPosition { x: coord_x, y: coord_y });
            }
        }
    }
}

fn period_to_slider(period: f32) -> f32 {
    (100.0 - 99.0 * (period - MIN_PERIOD) / (MAX_PERIOD - MIN_PERIOD)).clamp(1.0, 100.0)
}

fn slider_to_period(slider: f32) -> f32 {
    ((100.0 - slider) * (MAX_PERIOD - MIN_PERIOD) / 99.0 + MIN_PERIOD).clamp(MIN_PERIOD, MAX_PERIOD)
}

fn scale_to_slider(scale: f32) -> f32 {
    (1.0 + 99.0 * (scale - DEFAULT_SCALE) / (MAX_SCALE - DEFAULT_SCALE)).clamp(1.0, 100.0)
}

fn slider_to_scale(slider: f32) -> f32 {
    ((slider - 1.0) * (MAX_SCALE - DEFAULT_SCALE) / 99.0 + DEFAULT_SCALE)
        .clamp(DEFAULT_SCALE, MAX_SCALE)
}
