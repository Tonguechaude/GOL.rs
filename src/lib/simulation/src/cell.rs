//! # Cell Module
//!
//! Defines the basic cell types, components, and states for the Game of Life.

use bevy::prelude::*;

/// System set for organizing cell-related systems in the Bevy ECS.
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct CellSet;

/// Represents the position of a cell in the Game of Life grid.
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

/// Plugin for cell-related functionality
pub struct CellPlugin;

impl Plugin for CellPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DeadCellPool::default())
            .add_systems(Startup, setup_initial_pattern.in_set(CellSet));
    }
}

/// Sets up the initial pattern of living cells.
///
/// Spawns a simple pattern of cells to start the simulation.
/// This creates a small glider pattern that will move across the grid.
pub fn setup_initial_pattern(mut commands: Commands) {
    for &(x, y) in &[(0, 0), (-1, 0), (0, -1), (0, 1), (1, 1)] {
        commands.spawn((CellPosition { x, y }, Alive));
    }
}
