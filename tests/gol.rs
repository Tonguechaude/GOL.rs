use bevy::prelude::*;
use jeu_de_la_vie::cellule::*;

#[test]
fn test_setup_cellule() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_systems(Startup, setup_cellule)
        .update();

    let world_mut = app.world_mut();
    let mut query = world_mut.query::<&CellulePosition>();
    let cellules: Vec<_> = query.iter(&world_mut).collect();

    // Vérifier que les cellules initiales sont bien placées
    let expected_positions = vec![
        CellulePosition { x: 0, y: 0 },
        CellulePosition { x: -1, y: 0 },
        CellulePosition { x: 0, y: -1 },
        CellulePosition { x: 0, y: 1 },
        CellulePosition { x: 1, y: 1 },
    ];

    assert_eq!(cellules.len(), expected_positions.len());
    for pos in &expected_positions {
        assert!(cellules.contains(&pos), "Cellule absente : {:?}", pos);
    }
}
