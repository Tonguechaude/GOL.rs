//! # Input Module
//!
//! Handles keyboard and mouse input for camera movement and cell interaction.

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::config::{SimulationConfig, CameraConfig, DEFAULT_SCALE, BASE_SPEED, MAX_SPEED, ZOOM_STEP};
use crate::simulation::{CellPosition, Alive, DeadCellPool};

/// Resource to track the last painted position during drag operations
#[derive(Resource, Default)]
pub struct LastPaintedPosition {
    pub position: Option<CellPosition>,
}

/// Plugin for input handling systems
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LastPaintedPosition>()
            .add_systems(Update, (keyboard_input_system, mouse_click_system, reset_paint_position));
    }
}

/// Handles keyboard input for camera movement and simulation controls
pub fn keyboard_input_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut simulation_config: ResMut<SimulationConfig>,
    mut q_camera_transform: Query<&mut Transform, With<Camera>>,
    mut q_camera: Query<(&mut Projection, &GlobalTransform)>,
    q_cells: Query<Entity, With<Alive>>,
    mut dead_pool: ResMut<DeadCellPool>,
    time: Res<Time>,
    mut camera_config: ResMut<CameraConfig>,
) {
    let (mut x, mut y) = (0.0, 0.0);

    camera_config.turbo_mode = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);

    // Camera movement
    if keys.pressed(KeyCode::ArrowLeft) || keys.pressed(KeyCode::KeyH) {
        x -= 1.0;
    }
    if keys.pressed(KeyCode::ArrowRight) || keys.pressed(KeyCode::KeyL) {
        x += 1.0;
    }
    if keys.pressed(KeyCode::ArrowUp) || keys.pressed(KeyCode::KeyK) {
        y += 1.0;
    }
    if keys.pressed(KeyCode::ArrowDown) || keys.pressed(KeyCode::KeyJ) {
        y -= 1.0;
    }

    let Ok(mut transform) = q_camera_transform.single_mut() else {
        return;
    };

    let (mut camera_proj, _) = match q_camera.single_mut() {
        Ok(data) => data,
        Err(_) => return,
    };

    // Calculate movement speed based on camera scale
    let movement_speed = if let Projection::Orthographic(orthographic) = camera_proj.as_ref() {
        let base_speed = if camera_config.turbo_mode { MAX_SPEED } else { BASE_SPEED };
        let scale_factor = (orthographic.scale / DEFAULT_SCALE).clamp(0.1, 10.0);
        base_speed * scale_factor * time.delta_secs()
    } else {
        30.0 * time.delta_secs()
    };

    if x != 0.0 || y != 0.0 {
        let movement_vector = Vec3::new(x, y, 0.0).normalize_or_zero();
        transform.translation += movement_vector * movement_speed;
    }

    // Simulation controls
    if keys.just_pressed(KeyCode::Space) {
        simulation_config.running = !simulation_config.running;
    }
    if keys.just_pressed(KeyCode::KeyR) {
        simulation_config.running = false;
        clear_cells(&mut commands, &q_cells, &mut dead_pool);
    }
    if keys.just_pressed(KeyCode::KeyN) && !simulation_config.running {
        simulation_config.calculate_next_gen = true;
    }

    // Zoom controls
    if let Projection::Orthographic(orthographic) = camera_proj.as_mut() {
        if keys.just_pressed(KeyCode::KeyI) {
            orthographic.scale = (orthographic.scale / (1.0 + ZOOM_STEP)).max(DEFAULT_SCALE);
        }
        if keys.just_pressed(KeyCode::KeyO) {
            orthographic.scale = (orthographic.scale * (1.0 + ZOOM_STEP)).min(crate::config::MAX_SCALE);
        }
    }
}

/// Handles mouse clicks and drag to paint/erase cells
pub fn mouse_click_system(
    mut commands: Commands,
    simulation_config: Res<SimulationConfig>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    q_alive_cells: Query<(Entity, &CellPosition), With<Alive>>,
    q_dead_cells: Query<(Entity, &CellPosition), Without<Alive>>,
    mut dead_pool: ResMut<DeadCellPool>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut last_painted: ResMut<LastPaintedPosition>,
) {
    if simulation_config.running {
        return;
    }
    
    // Handle both click and drag (pressed instead of just_released)
    if !buttons.pressed(MouseButton::Left) {
        return;
    }
    
    let Ok(window) = q_windows.single() else {
        return;
    };
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };
    let Ok((camera, camera_transform)) = q_camera.single() else {
        return;
    };

    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };
    let position_cible = ray.origin.truncate().round();
    let new_cell = CellPosition { 
        x: position_cible.x as isize, 
        y: position_cible.y as isize 
    };
    
    // Skip if we already painted this position during the current drag
    if let Some(last_pos) = last_painted.position {
        if last_pos == new_cell {
            return;
        }
    }
    
    // Update the last painted position
    last_painted.position = Some(new_cell);
    
    // Check if there's a living cell at this position
    for (entity, cell_position) in q_alive_cells.iter() {
        if cell_position == &new_cell {
            commands.entity(entity)
                .remove::<Alive>()
                .insert(Visibility::Hidden);
            dead_pool.entities.push(entity);
            return;
        }
    }
    
    // Check if there's a dead cell at this position to revive
    for (entity, cell_position) in q_dead_cells.iter() {
        if cell_position == &new_cell {
            commands.entity(entity)
                .insert(Alive)
                .insert(Visibility::Visible);
            if let Some(index) = dead_pool.entities.iter().position(|&e| e == entity) {
                dead_pool.entities.swap_remove(index);
            }
            return;
        }
    }
    
    // No existing cell, try to reuse from pool or create new
    if let Some(entity) = dead_pool.entities.pop() {
        commands.entity(entity)
            .insert(new_cell)
            .insert(Alive)
            .insert(Visibility::Visible)
            .insert(Transform::from_xyz(new_cell.x as f32, new_cell.y as f32, 0.0));
    } else {
        commands.spawn((
            new_cell,
            Alive,
            Sprite {
                color: crate::config::CELL_COLOR,
                custom_size: Some(Vec2::new(1.0, 1.0)),
                ..Default::default()
            },
            Transform::from_xyz(new_cell.x as f32, new_cell.y as f32, 0.0),
            Visibility::Visible,
        ));
    }
}

/// Reset the last painted position when mouse button is released
pub fn reset_paint_position(
    buttons: Res<ButtonInput<MouseButton>>,
    mut last_painted: ResMut<LastPaintedPosition>,
) {
    if buttons.just_released(MouseButton::Left) {
        last_painted.position = None;
    }
}

/// Clear all cells
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