//! # Cell Module
//!
//! This module contains the core logic for Conway's Game of Life simulation.
//! It defines cell positions, simulation parameters, and the rules for cell
//! birth and death according to Conway's classic rules.

use bevy::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    time::Duration,
};

/// The eight neighboring positions relative to any cell.
/// These offsets represent the Moore neighborhood (all adjacent cells).
static NEIGHBORS: [(isize, isize); 8] =
    [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];

/// System set for organizing cell-related systems in the Bevy ECS.
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct CellSet;

/// Represents the position of a cell in the Game of Life grid.
///
/// Uses signed integers to allow for negative coordinates,
/// enabling an infinite grid that can expand in all directions.
#[derive(Clone, Component, PartialEq, Eq, Debug, Hash)]
pub struct CellPosition {
    /// The x-coordinate of the cell
    pub x: isize,
    /// The y-coordinate of the cell
    pub y: isize,
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
        Self { running: true, period: Duration::from_secs(1), calculate_next_gen: false }
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
        commands.spawn(CellPosition { x, y });
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
/// This system runs every frame and:
/// 1. Checks if it's time to calculate the next generation
/// 2. Counts neighbors for each cell and potential cell position
/// 3. Applies Conway's rules:
///    - Live cells with 2-3 neighbors survive
///    - Dead cells with exactly 3 neighbors become alive
///    - All other cells die or stay dead
/// 4. Spawns new cells and despawns dead ones
pub fn cell_system(
    mut commands: Commands,
    query: Query<(Entity, &CellPosition)>,
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

    let mut neighbors: HashMap<CellPosition, usize> = HashMap::new();
    let mut spawn_candidates: HashSet<CellPosition> = HashSet::new();
    let mut cells_to_remove = Vec::new();

    for (_, cell) in &query {
        for &(dx, dy) in &NEIGHBORS {
            let neighbor_pos = CellPosition { x: cell.x + dx, y: cell.y + dy };
            let neighbor_count = neighbors.entry(neighbor_pos.clone()).or_insert(0);
            *neighbor_count += 1;
            if *neighbor_count == 3 {
                spawn_candidates.insert(neighbor_pos);
            } else if *neighbor_count == 4 {
                spawn_candidates.remove(&neighbor_pos);
            } else {
                // No action needed for other neighbor counts
            }
        }
    }

    for (entity, cell) in &query {
        match neighbors.get(cell).copied().unwrap_or(0) {
            0 | 1 => cells_to_remove.push(entity),
            2 => (),
            3 => {
                spawn_candidates.remove(cell);
            }
            _ => cells_to_remove.push(entity),
        }
    }

    for entity in cells_to_remove {
        commands.entity(entity).despawn();
    }

    for new_cell in spawn_candidates {
        commands.spawn(new_cell);
    }
}
