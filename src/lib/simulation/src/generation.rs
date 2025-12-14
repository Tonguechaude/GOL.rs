//! # Generation Module
//!
//! Handles the main simulation loop, timing, and generation calculations.

use bevy::prelude::{
    App, Commands, DetectChanges, Entity, IntoScheduleConfigs, Plugin, Query, Res, ResMut,
    Resource, Time, Timer, TimerMode, Transform, Update, Visibility, With,
};
use rustc_hash::FxHashSet;

use crate::cell::{Alive, CellPosition, CellSet, DeadCellPool};
use crate::rules::{calculate_neighbor_counts, should_cell_be_born, should_cell_survive};
use gol_config::SimulationConfig;

/// Timer resource that controls when to calculate the next generation.
///
/// Wraps a Bevy Timer to track when enough time has passed
/// for the next generation update.
#[derive(Resource)]
pub struct GenerationTimer(pub Timer);

/// Plugin for generation calculation systems
pub struct GenerationPlugin;

impl Plugin for GenerationPlugin {
    fn build(&self, app: &mut App) {
        let config = SimulationConfig::default();
        let period = config.period;
        app.insert_resource(GenerationTimer(Timer::new(period, TimerMode::Repeating)))
            .add_systems(Update, simulation_config_listener)
            .add_systems(Update, calculate_next_generation.in_set(CellSet));
    }
}

/// Listens for changes to simulation parameters and updates the timer accordingly.
///
/// When the simulation speed (period) is changed, this system updates
/// the generation timer to use the new duration.
pub fn simulation_config_listener(
    config: Res<SimulationConfig>,
    mut timer: ResMut<GenerationTimer>,
) {
    if config.is_changed() {
        if config.period != timer.0.duration() {
            timer.0.set_duration(config.period);
            timer.0.reset();
        }
    }
}

/// Main system that implements Conway's Game of Life rules.
///
/// Applies Conway's rules:
///  - Live cells with 2-3 neighbors survive
///  - Dead cells with exactly 3 neighbors become alive
///  - All other cells die or stay dead
pub fn calculate_next_generation(
    mut commands: Commands,
    alive_query: Query<(Entity, &CellPosition), With<Alive>>,
    mut dead_pool: ResMut<DeadCellPool>,
    mut timer: ResMut<GenerationTimer>,
    mut config: ResMut<SimulationConfig>,
    time: Res<Time>,
) {
    if config.running {
        timer.0.tick(time.delta());
        if !timer.0.finished() {
            return;
        }
    } else if !config.calculate_next_gen {
        return;
    } else {
        config.calculate_next_gen = false;
    }

    let cell_count = alive_query.iter().count();

    // Pre-allocation for performance
    let mut cells_to_kill = Vec::with_capacity(cell_count / 2);
    // Create set of alive positions for quick lookup
    let alive_positions: FxHashSet<CellPosition> =
        alive_query.iter().map(|(_, pos)| *pos).collect();

    // Calculate neighbor counts for all relevant positions
    let neighbor_counts = calculate_neighbor_counts(alive_positions.iter().copied());

    // Determine which cells should die
    for (entity, cell) in &alive_query {
        let neighbor_count = neighbor_counts.get(cell).copied().unwrap_or(0);
        if !should_cell_survive(neighbor_count) {
            cells_to_kill.push(entity);
        }
    }

    // Determine which cells should be born
    let mut cells_to_spawn = Vec::new();
    for (pos, count) in &neighbor_counts {
        if should_cell_be_born(*count) && !alive_positions.contains(pos) {
            cells_to_spawn.push(*pos);
        }
    }

    // Kill cells
    for entity in cells_to_kill {
        commands
            .entity(entity)
            .remove::<Alive>()
            .insert(Visibility::Hidden);
        dead_pool.entities.push(entity);
    }

    // Spawn new cells
    for new_pos in cells_to_spawn {
        if let Some(entity) = dead_pool.entities.pop() {
            commands
                .entity(entity)
                .insert(Alive)
                .insert(Visibility::Visible)
                .insert(Transform::from_xyz(new_pos.x as f32, new_pos.y as f32, 0.0))
                .insert(new_pos);
        } else {
            commands.spawn((new_pos, Alive, Visibility::Visible));
        }
    }
}
