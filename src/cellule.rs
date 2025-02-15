use std::{collections::{HashSet, HashMap}, time::Duration};
use bevy::prelude::*;

static VOISINS: [(isize, isize); 8] = [
    (-1, -1), (0, -1), (1, -1), (-1, 0),
    (1, 0), (-1, 1), (0, 1), (1, 1),
];

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct CelluleSet;

#[derive(Clone, Component, PartialEq, Eq, Debug, Hash)]
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
    for &(x, y) in &[ (0, 0), (-1, 0), (0, -1), (0, 1), (1, 1) ] {
        commands.spawn(CellulePosition { x, y });
    }
}

fn cellule_params_listener(my_res: Res<CelluleParams>, mut timer: ResMut<TimerNouvelleGen>) {
    if my_res.is_changed() {
        debug!("CelluleParams mis à jour : {:?}", *my_res);
        if my_res.periode != timer.0.duration() {
            timer.0.set_duration(my_res.periode);
            timer.0.reset();
        }
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
            let voisin_pos = CellulePosition { x: cell.x + dx, y: cell.y + dy };
            let nb_voisins = voisins.entry(voisin_pos.clone()).or_insert(0);
            *nb_voisins += 1;
            if *nb_voisins == 3 {
                spawn_candidates.insert(voisin_pos);
            } else if *nb_voisins == 4 {
                spawn_candidates.remove(&voisin_pos);
            }
        }
    }
   
    for (entity, cellule) in &query {
        match voisins.get(cellule).copied().unwrap_or(0) {
            0 | 1 => cellules_a_supprimer.push(entity),
            2 => (),
            3 => { spawn_candidates.remove(cellule); },
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