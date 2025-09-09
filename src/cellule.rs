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
static VOISINS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

/// System set for organizing cell-related systems in the Bevy ECS.
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct CelluleSet;

/// Represents the position of a cell in the Game of Life grid.
/// 
/// Uses signed integers to allow for negative coordinates,
/// enabling an infinite grid that can expand in all directions.
#[derive(Clone, Component, PartialEq, Eq, Debug, Hash)]
pub struct CellulePosition {
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
pub struct CelluleParams {
    /// Whether the simulation is currently running automatically
    pub en_cours: bool,
    /// Time delay between each generation update
    pub periode: Duration,
    /// Flag to trigger a single step calculation when the simulation is paused
    pub calcule_prochaine_gen: bool,
}

impl Default for CelluleParams {
    fn default() -> Self {
        Self {
            en_cours: true,
            periode: Duration::from_secs(1),
            calcule_prochaine_gen: false,
        }
    }
}

/// Timer resource that controls when to calculate the next generation.
/// 
/// Wraps a Bevy Timer to track when enough time has passed
/// for the next generation update.
#[derive(Resource)]
pub struct TimerNouvelleGen(Timer);

/// Bevy plugin that sets up the Game of Life simulation systems.
/// 
/// This plugin initializes all necessary resources and systems
/// for running Conway's Game of Life within a Bevy application.
pub struct CelluleSystem;

impl Plugin for CelluleSystem {
    fn build(&self, app: &mut App) {
        let cellule_params = CelluleParams::default();
        let periode = cellule_params.periode;
        app.insert_resource(cellule_params)
            .insert_resource(TimerNouvelleGen(Timer::new(periode, TimerMode::Repeating)))
            .add_systems(Update, cellule_params_listener)
            .add_systems(Startup, setup_cellule.in_set(CelluleSet))
            .add_systems(Update, system_cellules.in_set(CelluleSet));
    }
}

/// Sets up the initial pattern of living cells.
/// 
/// Spawns a simple pattern of cells to start the simulation.
/// This creates a small glider pattern that will move across the grid.
pub fn setup_cellule(mut commands: Commands) {
    for &(x, y) in &[(0, 0), (-1, 0), (0, -1), (0, 1), (1, 1)] {
        commands.spawn(CellulePosition { x, y });
    }
}

/// Listens for changes to simulation parameters and updates the timer accordingly.
/// 
/// When the simulation speed (period) is changed, this system updates
/// the generation timer to use the new duration.
pub fn cellule_params_listener(my_res: Res<CelluleParams>, mut timer: ResMut<TimerNouvelleGen>) {
    if my_res.is_changed() {
        debug!("CelluleParams updated: {:?}", *my_res);
        if my_res.periode != timer.0.duration() {
            timer.0.set_duration(my_res.periode);
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
pub fn system_cellules(
    mut commands: Commands,
    query: Query<(Entity, &CellulePosition)>,
    mut timer: ResMut<TimerNouvelleGen>,
    mut cellule_params: ResMut<CelluleParams>,
    time: Res<Time>,
) {
    if cellule_params.en_cours {
        timer.0.tick(time.delta());
        if !timer.0.finished() {
            return;
        }
    } else if !cellule_params.calcule_prochaine_gen {
        return;
    } else {
        cellule_params.calcule_prochaine_gen = false;
    }

    let mut voisins: HashMap<CellulePosition, usize> = HashMap::new();
    let mut spawn_candidates: HashSet<CellulePosition> = HashSet::new();
    let mut cellules_a_supprimer = Vec::new();

    for (_, cell) in &query {
        for &(dx, dy) in &VOISINS {
            let voisin_pos = CellulePosition {
                x: cell.x + dx,
                y: cell.y + dy,
            };
            let nb_voisins = voisins.entry(voisin_pos.clone()).or_insert(0);
            *nb_voisins += 1;
            if *nb_voisins == 3 {
                spawn_candidates.insert(voisin_pos);
            } else if *nb_voisins == 4 {
                spawn_candidates.remove(&voisin_pos);
            } else {
                // Pas d'action nécessaire pour les autres nombres de voisins
            }
        }
    }

    for (entity, cellule) in &query {
        match voisins.get(cellule).copied().unwrap_or(0) {
            0 | 1 => cellules_a_supprimer.push(entity),
            2 => (),
            3 => {
                spawn_candidates.remove(cellule);
            }
            _ => cellules_a_supprimer.push(entity),
        }
    }

    for entity in cellules_a_supprimer {
        commands.entity(entity).despawn();
    }

    for nouvelle_cellule in spawn_candidates {
        commands.spawn(nouvelle_cellule);
    }
}
