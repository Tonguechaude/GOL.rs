//! # Cell Module
//!
//! This module contains the core logic for Conway's Game of Life simulation.
//! It defines cell positions, simulation parameters, and the rules for cell
//! birth and death according to Conway's classic rules.

use bevy::prelude::*;
use rustc_hash::{FxHashMap, FxHashSet};
use std::time::Duration;

/// The eight neighboring positions relative to any cell.
/// These offsets represent the Moore neighborhood (all adjacent cells).
static NEIGHBORS: [(isize, isize); 8] =
    [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];

/// System set for organizing cell-related systems in the Bevy ECS.
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct CellSet;

/// Represents the position of a cell in the Game of Life grid.
/// 
///
/// Uses signed integers to allow for negative coordinates,
/// enabling an infinite grid that can expand in all directions.
#[derive(Clone, Copy, Component, PartialEq, Eq, Debug, Hash)]
pub struct CellPosition {
    /// The x-coordinate of the cell
    pub x: isize,
    /// The y-coordinate of the cell
    pub y: isize,
}

/// Marker component for cells that are currently alive
#[derive(Component)]
pub struct Alive;

/// Pool of dead cell entities ready for reuse
#[derive(Resource, Default)]
pub struct DeadCellPool {
    pub entities: Vec<Entity>,
}

/// Configuration parameters for the Game of Life simulation.
///
/// This resource controls the behavior of the simulation including
/// whether it's running automatically and at what speed.
#[derive(Resource, Debug)]
pub struct CellParams {
    /// Whether the simulation is currently running automatically
    pub running: bool,
    /// Time delay between each generation update
    pub period: Duration,
    /// Flag to trigger a single step calculation when the simulation is paused
    pub calculate_next_gen: bool,
}

impl Default for CellParams {
    fn default() -> Self {
        Self { 
            running: true, 
            period: Duration::from_secs(1), 
            calculate_next_gen: false 
        }
    }
}

/// Timer resource that controls when to calculate the next generation.
/// 
/// Wraps a Bevy Timer to track when enough time has passed
/// for the next generation update.
#[derive(Resource)]
pub struct NewGenTimer(Timer);

/// Bevy plugin that sets up the Game of Life simulation systems.
///
/// This plugin initializes all necessary resources and systems
/// for running Conway's Game of Life within a Bevy application.
pub struct CellSystem;

impl Plugin for CellSystem {
    fn build(&self, app: &mut App) {
        let cell_params = CellParams::default();
        let period = cell_params.period;
        app.insert_resource(cell_params)
            .insert_resource(NewGenTimer(Timer::new(period, TimerMode::Repeating)))
            .insert_resource(DeadCellPool::default())
            .add_systems(Update, cell_params_listener)
            .add_systems(Startup, setup_cells.in_set(CellSet))
            .add_systems(Update, cell_system.in_set(CellSet));
    }
}

/// Sets up the initial pattern of living cells.
///
/// Spawns a simple pattern of cells to start the simulation.
/// This creates a small glider pattern that will move across the grid.
pub fn setup_cells(mut commands: Commands) {
    for &(x, y) in &[(0, 0), (-1, 0), (0, -1), (0, 1), (1, 1)] {
        commands.spawn((
            CellPosition { x, y },
            Alive,
        ));
    }
}

/// Listens for changes to simulation parameters and updates the timer accordingly.
///
/// When the simulation speed (period) is changed, this system updates
/// the generation timer to use the new duration.
pub fn cell_params_listener(my_res: Res<CellParams>, mut timer: ResMut<NewGenTimer>) {
    if my_res.is_changed() {
        dbg!("CellParams updated: {:?}", &my_res);
        if my_res.period != timer.0.duration() {
            timer.0.set_duration(my_res.period);
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
pub fn cell_system(
    mut commands: Commands,
    alive_query: Query<(Entity, &CellPosition), With<Alive>>,
    mut dead_pool: ResMut<DeadCellPool>,
    mut timer: ResMut<NewGenTimer>,
    mut cell_params: ResMut<CellParams>,
    time: Res<Time>,
) {
    if cell_params.running {
        timer.0.tick(time.delta());
        if !timer.0.finished() {
            return;
        }
    } else if !cell_params.calculate_next_gen {
        return;
    } else {
        cell_params.calculate_next_gen = false;
    }

    let cell_count = alive_query.iter().count();
    
    // Pre-allocation
    let mut neighbors: FxHashMap<CellPosition, usize> = 
        FxHashMap::with_capacity_and_hasher(cell_count * 9, Default::default());
    let mut cells_to_kill = Vec::with_capacity(cell_count / 2);

    let mut alive_positions: FxHashSet<CellPosition> = 
        FxHashSet::with_capacity_and_hasher(cell_count, Default::default());
    
    for (_, pos) in &alive_query {
        alive_positions.insert(*pos);
    }

    for (_, cell) in &alive_query {
        for &(dx, dy) in &NEIGHBORS {
            let neighbor_pos = CellPosition { x: cell.x + dx, y: cell.y + dy };
            *neighbors.entry(neighbor_pos).or_insert(0) += 1;
        }
    }

    for (entity, cell) in &alive_query {
        match neighbors.get(cell).copied().unwrap_or(0) {
            2 | 3 => (),
            _ => cells_to_kill.push(entity),
        }
    }

    let mut cells_to_spawn = Vec::new();
    for (pos, count) in &neighbors {
        if *count == 3 && !alive_positions.contains(pos) {
            cells_to_spawn.push(*pos);
        }
    }

    for entity in cells_to_kill {
        commands.entity(entity)
            .remove::<Alive>()
            .insert(Visibility::Hidden);
        dead_pool.entities.push(entity);
    }

    for new_pos in cells_to_spawn {
        if let Some(entity) = dead_pool.entities.pop() {
            commands.entity(entity)
                .insert(Alive)
                .insert(Visibility::Visible)
                .insert(Transform::from_xyz(new_pos.x as f32, new_pos.y as f32, 0.0))
                .insert(new_pos);
        } else {
            commands.spawn((
                new_pos,
                Alive,
                Visibility::Visible,
            ));
        }
    }
}
