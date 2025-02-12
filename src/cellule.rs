use std::{collections::BTreeSet, time::Duration};

use bevy::{prelude::*, utils::HashMap};

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

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct CelluleSet;

#[derive(Clone, Component, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
pub struct CellulePosition {
    pub x: isize,
    pub y: isize,
}

#[derive(Resource, Debug)]
pub struct CelluleParams {
    pub en_cours: bool,
    pub periode: Duration,
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

#[derive(Resource)]
pub struct TimerNouvelleGen(Timer);

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

fn setup_cellule(mut commands: Commands) {
    commands.spawn(CellulePosition { x: 0, y: 0 });
    commands.spawn(CellulePosition { x: -1, y: 0 });
    commands.spawn(CellulePosition { x: 0, y: -1 });
    commands.spawn(CellulePosition { x: 0, y: 1 });
    commands.spawn(CellulePosition { x: 1, y: 1 });
}

fn cellule_params_listener(my_res: Res<CelluleParams>, mut timer: ResMut<TimerNouvelleGen>) {
    if !my_res.is_changed() {
        return;
    }
    debug!("CelluleParams est passée en mode : {:?}", *my_res);
    if my_res.periode != timer.0.duration() {
        timer.0.set_duration(my_res.periode);
        timer.0.reset();
    }
}

fn system_cellules(
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
    } else if cellule_params.calcule_prochaine_gen {
        cellule_params.calcule_prochaine_gen = false;
    } else {
        return;
    }
    let mut voisins = HashMap::new();
    let mut spawn_candidates = BTreeSet::new();
    for (_, cell) in &query {
        for pos_delta in VOISINS.iter() {
            let tmp_position = CellulePosition {
                x: cell.x + pos_delta.0,
                y: cell.y + pos_delta.1,
            };
            let nb_voisins = match voisins.get(&tmp_position) {
                Some(prev_val) => prev_val + 1,
                None => 1,
            };
            voisins.insert(tmp_position.clone(), nb_voisins);
            if nb_voisins == 3 {
                spawn_candidates.insert(tmp_position.clone());
            } else if nb_voisins == 4 {
                spawn_candidates.remove(&tmp_position);
            }
        }
    }
    for (entity, cellule) in &query {
        let nb_voisins = *voisins.get(cellule).unwrap_or(&0);
        match nb_voisins {
            0..=1 => commands.entity(entity).despawn(),
            2 => (),
            3 => {
                spawn_candidates.remove(cellule);
            }
            _ => commands.entity(entity).despawn(),
        }
    }
    for nouvelle_cellule in spawn_candidates {
        commands.spawn(nouvelle_cellule);
    }
}

