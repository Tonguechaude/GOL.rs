//! # Rules Module
//!
//! Implements Conway's Game of Life rules and neighbor calculations.

use super::cell::CellPosition;
use rustc_hash::FxHashMap;

/// The eight neighboring positions relative to any cell.
/// These offsets represent the Moore neighborhood (all adjacent cells).
pub static NEIGHBORS: [(isize, isize); 8] =
    [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];

/// Calculates neighbor counts for all relevant positions
///
/// Returns a map of positions to their neighbor counts, including both
/// alive cells and their neighboring empty positions that might become alive.
pub fn calculate_neighbor_counts<'a, I>(alive_cells: I) -> FxHashMap<CellPosition, usize>
where
    I: Iterator<Item = CellPosition> + Clone,
{
    let cell_count = alive_cells.clone().count();
    let mut neighbors: FxHashMap<CellPosition, usize> = 
        FxHashMap::with_capacity_and_hasher(cell_count * 9, Default::default());

    for cell in alive_cells {
        for &(dx, dy) in &NEIGHBORS {
            let neighbor_pos = CellPosition { x: cell.x + dx, y: cell.y + dy };
            *neighbors.entry(neighbor_pos).or_insert(0) += 1;
        }
    }

    neighbors
}

/// Determines if a cell should survive based on Conway's rules
///
/// - Live cells with 2-3 neighbors survive
/// - All other live cells die
pub fn should_cell_survive(neighbor_count: usize) -> bool {
    matches!(neighbor_count, 2 | 3)
}

/// Determines if a cell should be born based on Conway's rules
///
/// - Dead cells with exactly 3 neighbors become alive
pub fn should_cell_be_born(neighbor_count: usize) -> bool {
    neighbor_count == 3
}