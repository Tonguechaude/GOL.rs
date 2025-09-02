use std::time::Duration;

use crate::cellule::{CelluleParams, CellulePosition, CelluleSet};
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{
    EguiContexts, EguiPlugin,
    egui::{self, Color32, Ui},
};
use egui_modal::Modal;
use rand::Rng;

type Seconds = f32;

const COULEUR_BG: Color = Color::srgb(0.9, 0.9, 0.9);
const COULEUR_CELLULE: Color = Color::srgb(0.0, 0.0, 0.0);

const ECHELLE_DEFAUT: f32 = 1.0 / 40.0;
const ECHELLE_MAX: f32 = 1.0;

const PERIODE_MIN: Seconds = 0.01;
const PERIODE_MAX: Seconds = 1.5;

pub struct GuiSystem;

impl Plugin for GuiSystem {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(COULEUR_BG))
            .insert_resource(GuiParams::default())
            .add_plugins(EguiPlugin)
            .add_systems(Startup, init_camera)
            .add_systems(Update, system_gui)
            .add_systems(Update, system_clique_souris)
            .add_systems(Update, system_entree_clavier)
            .add_systems(Update, system_dessiner_nouvelle_cellules.before(CelluleSet))
            .add_systems(
                Update,
                system_dessin_grille
                    .after(system_dessiner_nouvelle_cellules)
                    .run_if(|gui_params: Res<GuiParams>| gui_params.grille_active),
            );
    }
}

#[derive(Resource, Debug)]
pub struct GuiParams {
    pub largeur_grille_gen_aleatoire: u16,
    pub grille_active: bool,
}

impl Default for GuiParams {
    fn default() -> Self {
        Self {
            largeur_grille_gen_aleatoire: 50_u16,
            grille_active: true,
        }
    }
}

fn init_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            scale: ECHELLE_DEFAUT,
            far: 1000.0,
            near: -1000.0,
            ..OrthographicProjection::default_2d()
        },
    ));
}

fn system_gui(
    mut commands: Commands,
    mut contexts: EguiContexts,
    mut cellule_params: ResMut<CelluleParams>,
    mut gui_params: ResMut<GuiParams>,
    mut q_camera: Query<(&mut OrthographicProjection, &GlobalTransform)>,
    q_cells: Query<Entity, With<CellulePosition>>,
) {
    let ctx = contexts.ctx_mut();
    ctx.set_visuals(egui::style::Visuals::light());
    let (mut camera_proj, camera_transform) = match q_camera.get_single_mut() {
        Ok(data) => data,
        Err(_) => return,
    };
    let speed_slider_init = periode_to_slider(cellule_params.periode.as_secs_f32());
    let mut vitesse_slider = speed_slider_init;
    let scale_slider_init = echelle_to_slider(camera_proj.scale);
    let mut scale_slider_val = scale_slider_init;

    let reset_modal = {
        let modal = Modal::new(ctx, "reset_modal");
        modal.show(|ui| {
            modal.title(ui, "Tuer toutes les cellules !!");
            modal.frame(ui, |ui| {
                modal.body(ui, "On tue tout le monde ?");
            });
            modal.buttons(ui, |ui| {
                modal.button(ui, "Non");
                if modal.button(ui, "Oui").clicked() {
                    cellule_params.en_cours = false;
                    nettoyage_cellules(&mut commands, &q_cells);
                };
            });
        });
        modal
    };
    let random_modal = {
        let modal = Modal::new(ctx, "random_modal");
        modal.show(|ui| {
            modal.title(ui, "Génération aléatoire");
            modal.frame(ui, |ui| {
                modal.body(ui, "Remplir aléatoirement la grille ?");
            });
            modal.buttons(ui, |ui| {
                modal.button(ui, "Non");
                if modal.button(ui, "Oui").clicked() {
                    let offset = -(gui_params.largeur_grille_gen_aleatoire as isize) / 2;
                    let width = gui_params.largeur_grille_gen_aleatoire as usize;
                    nettoyage_cellules(&mut commands, &q_cells);
                    generation_alleatoire_cellule(&mut commands, offset, offset, width, width);
                };
            });
        });
        modal
    };
    let separateur = |ui: &mut Ui| ui.add(egui::Separator::default());

    egui::Window::new("Jeu de la Vie")
        .resizable(false)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Nettoyer la grille").clicked() {
                    reset_modal.open();
                }
            });
            ui.horizontal(|ui| {
                ui.add(
                    egui::DragValue::new(&mut gui_params.largeur_grille_gen_aleatoire)
                        .suffix(" largeur"),
                );
                if ui.button("Cellule aléatoire").clicked() {
                    random_modal.open();
                }
            });
            separateur(ui);
            ui.vertical(|ui| {
                ui.add(
                    egui::Slider::new(&mut vitesse_slider, 1.0..=100.0)
                        .text("Vitesse")
                        .show_value(false),
                );
                ui.add(
                    egui::Slider::new(&mut scale_slider_val, 1.0..=100.0)
                        .text("Distance caméra")
                        .show_value(false)
                        .logarithmic(true),
                );
            });
            separateur(ui);
            ui.horizontal(|ui| {
                let play_text = if cellule_params.en_cours {
                    "Pause"
                } else {
                    "Lancer"
                };
                if ui.button(play_text).clicked() {
                    cellule_params.en_cours = !cellule_params.en_cours;
                }
                let next_step_btn = ui.add_enabled(
                    !cellule_params.en_cours,
                    egui::Button::new("Prochaine génération"),
                );
                if !cellule_params.en_cours && next_step_btn.clicked() {
                    cellule_params.calcule_prochaine_gen = true;
                };
            });
            separateur(ui);
            ui.vertical(|ui| {
                ui.checkbox(&mut gui_params.grille_active, "Afficher le quadrillage");
            });
            separateur(ui);
            ui.vertical(|ui| {
                let x = camera_transform.translation().x;
                let y = camera_transform.translation().y;
                ui.label(format!("Position actuelle : x: {x}, y: {y}"));
                ui.add_space(5.);
                ui.label("Tu peux appuyer sur la grille quand la génération est en pause !");
                ui.label("Utilise les fléches directionnelles pour te déplacer !");
            });
        });

    if scale_slider_init != scale_slider_val {
        camera_proj.scale = slider_to_echelle(scale_slider_val);
    }
    if speed_slider_init != vitesse_slider {
        cellule_params.periode = Duration::from_secs_f32(slider_to_periode(vitesse_slider));
    }
}

fn system_dessiner_nouvelle_cellules(
    mut commands: Commands,
    query: Query<(Entity, &CellulePosition), Added<CellulePosition>>,
) {
    for (entity, pos) in query.iter() {
        commands.entity(entity).insert(Sprite {
            color: COULEUR_CELLULE,
            custom_size: Some(Vec2::new(1.0, 1.0)),
            ..Default::default()
        });

        commands
            .entity(entity)
            .insert(Transform::from_xyz(pos.x as f32, pos.y as f32, 0.0));
    }
}

fn system_clique_souris(
    mut commands: Commands,
    cellule_params: Res<CelluleParams>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    q_cellpos: Query<(Entity, &CellulePosition)>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    if cellule_params.en_cours || !buttons.just_released(MouseButton::Left) {
        return;
    }
    let Some(cursor_position) = q_windows.single().cursor_position() else {
        return;
    };
    let (camera, camera_transform) = q_camera.single();

    let position_cible = match camera.viewport_to_world(camera_transform, cursor_position) {
        Ok(ray) => ray.origin.truncate().round(),
        Err(_) => {
            eprintln!("T'as cliqué où la bon sang !");
            return;
        }
    };

    debug!("Position du clique : {position_cible}");
    let nouvelle_cellule = CellulePosition {
        x: position_cible.x as isize,
        y: position_cible.y as isize,
    };
    for (entity, position_cellule) in q_cellpos.iter() {
        if position_cellule == &nouvelle_cellule {
            commands.entity(entity).despawn();
            return;
        }
    }
    commands.spawn(nouvelle_cellule);
}

fn system_entree_clavier(
    keys: Res<ButtonInput<KeyCode>>,
    mut q_camera_transform: Query<&mut Transform, With<Camera>>,
) {
    let (mut x, mut y) = (0, 0);
    if keys.pressed(KeyCode::ArrowLeft) {
        x += -1;
    }
    if keys.pressed(KeyCode::ArrowRight) {
        x += 1;
    }
    if keys.pressed(KeyCode::ArrowUp) {
        y += 1;
    }
    if keys.pressed(KeyCode::ArrowDown) {
        y += -1;
    }
    let mut transform = q_camera_transform.single_mut();
    transform.translation += Vec3::new(x as f32, y as f32, 0.0);
}

fn system_dessin_grille(
    mut contexts: EguiContexts,
    q_camera: Query<(&Camera, &OrthographicProjection, &GlobalTransform)>,
) {
    const COULEUR_LIGNE: Color32 = Color32::BLACK;
    let (camera, camera_proj, camera_transform) = match q_camera.get_single() {
        Ok(data) => data,
        Err(_) => return,
    };
    let ctx = contexts.ctx_mut();
    let image_transparente = egui::containers::Frame {
        fill: Color32::TRANSPARENT,
        ..Default::default()
    };
    let largeur_ligne =
        (1.0 - (camera_proj.scale - ECHELLE_DEFAUT) / (ECHELLE_MAX - ECHELLE_DEFAUT)).powi(10);

    egui::CentralPanel::default()
        .frame(image_transparente)
        .show(ctx, |ui| {
            let (response, painter) = ui.allocate_painter(
                bevy_egui::egui::Vec2::new(ui.available_width(), ui.available_height()),
                egui::Sense {
                    click: false,
                    drag: false,
                    focusable: false,
                },
            );
            let visible_top_left = camera
                .viewport_to_world(camera_transform, Vec2 { x: 0.0, y: 0.0 })
                .map(|ray| ray.origin.truncate())
                .unwrap();
            let (x_min, y_max) = (
                visible_top_left.x.round() as isize,
                visible_top_left.y.round() as isize,
            );
            let visible_bottom_right = camera
                .viewport_to_world(
                    camera_transform,
                    Vec2 {
                        x: response.rect.right(),
                        y: response.rect.bottom(),
                    },
                )
                .map(|ray| ray.origin.truncate())
                .unwrap();
            let (x_max, y_min) = (
                visible_bottom_right.x.round() as isize,
                visible_bottom_right.y.round() as isize,
            );
            for x in x_min..=x_max {
                let start = camera
                    .world_to_viewport(
                        camera_transform,
                        Vec3 {
                            x: x as f32 - 0.5,
                            y: y_min as f32 - 0.5,
                            z: 0.0,
                        },
                    )
                    .unwrap();
                let debut = egui::Pos2::new(start.x, start.y);
                let fin = camera
                    .world_to_viewport(
                        camera_transform,
                        Vec3 {
                            x: x as f32 - 0.5,
                            y: y_max as f32 + 0.5,
                            z: 0.0,
                        },
                    )
                    .unwrap();
                let fin = egui::Pos2::new(fin.x, fin.y);
                painter.add(egui::Shape::LineSegment {
                    points: [debut, fin],
                    stroke: egui::Stroke {
                        width: largeur_ligne,
                        color: COULEUR_LIGNE,
                    }
                    .into(),
                });
            }
            for y in y_min..=y_max {
                let debut = camera
                    .world_to_viewport(
                        camera_transform,
                        Vec3 {
                            x: x_min as f32 - 0.5,
                            y: y as f32 - 0.5,
                            z: 0.0,
                        },
                    )
                    .unwrap();
                let debut = egui::Pos2::new(debut.x, debut.y);
                let fin = camera
                    .world_to_viewport(
                        camera_transform,
                        Vec3 {
                            x: x_max as f32 + 0.5,
                            y: y as f32 - 0.5,
                            z: 0.0,
                        },
                    )
                    .unwrap();
                let fin = egui::Pos2::new(fin.x, fin.y);
                painter.add(egui::Shape::LineSegment {
                    points: [debut, fin],
                    stroke: egui::Stroke {
                        width: largeur_ligne,
                        color: COULEUR_LIGNE,
                    }
                    .into(),
                });
            }
        });
}

fn nettoyage_cellules(commands: &mut Commands, q_cellules: &Query<Entity, With<CellulePosition>>) {
    let cellules_a_supprimer: Vec<Entity> = q_cellules.iter().collect();
    for entity in cellules_a_supprimer {
        commands.entity(entity).despawn();
    }
}

fn generation_alleatoire_cellule(
    commands: &mut Commands,
    x: isize,
    y: isize,
    largeur: usize,
    hauteur: usize,
) {
    let mut rng = rand::rng();
    for coord_x in x..(x + largeur as isize) {
        for coord_y in y..(y + hauteur as isize) {
            if rng.random::<bool>() {
                commands.spawn(CellulePosition {
                    x: coord_x,
                    y: coord_y,
                });
            }
        }
    }
}

fn periode_to_slider(period: f32) -> f32 {
    (100.0 - 99.0 * (period - PERIODE_MIN) / (PERIODE_MAX - PERIODE_MIN)).clamp(1.0, 100.0)
}

fn slider_to_periode(slider: f32) -> f32 {
    ((100.0 - slider) * (PERIODE_MAX - PERIODE_MIN) / 99.0 + PERIODE_MIN)
        .clamp(PERIODE_MIN, PERIODE_MAX)
}

fn echelle_to_slider(scale: f32) -> f32 {
    (1.0 + 99.0 * (scale - ECHELLE_DEFAUT) / (ECHELLE_MAX - ECHELLE_DEFAUT)).clamp(1.0, 100.0)
}

fn slider_to_echelle(slider: f32) -> f32 {
    ((slider - 1.0) * (ECHELLE_MAX - ECHELLE_DEFAUT) / 99.0 + ECHELLE_DEFAUT)
        .clamp(ECHELLE_DEFAUT, ECHELLE_MAX)
}
