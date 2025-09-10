use bevy::prelude::*;
use jeu_de_la_vie::cellule::*;

#[test]
fn test_setup_cells() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins).add_systems(Startup, setup_cells).update();

    let world_mut = app.world_mut();
    let mut query = world_mut.query::<&CellPosition>();
    let cells: Vec<_> = query.iter(&world_mut).collect();

    // Verify that initial cells are correctly placed
    let expected_positions = vec![
        CellPosition { x: 0, y: 0 },
        CellPosition { x: -1, y: 0 },
        CellPosition { x: 0, y: -1 },
        CellPosition { x: 0, y: 1 },
        CellPosition { x: 1, y: 1 },
    ];

    assert_eq!(cells.len(), expected_positions.len());
    for pos in &expected_positions {
        assert!(cells.contains(&pos), "Missing cell: {:?}", pos);
    }
}

#[test]
fn test_update_cells2() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins).add_plugins(CellSystem);

    app.update();

    {
        let mut cell_params = app.world_mut().resource_mut::<CellParams>();
        cell_params.running = false; // Stop the timer
        cell_params.calculate_next_gen = true; // Force execution
    }

    app.update();

    let world = app.world_mut();
    let mut query = world.query::<&CellPosition>();
    let cells: Vec<_> = query.iter(world).collect();

    println!("Cells after 1 iteration: {:?}", cells);

    let expected_positions = vec![
        CellPosition { x: 0, y: -1 },
        CellPosition { x: -1, y: -1 },
        CellPosition { x: -1, y: -1 },
        CellPosition { x: -1, y: 1 },
        CellPosition { x: 0, y: 1 },
        CellPosition { x: 1, y: 1 },
    ];

    assert_eq!(cells.len(), expected_positions.len());
    for pos in &expected_positions {
        assert!(cells.contains(&pos), "Missing cell: {:?}", pos);
    }
}
