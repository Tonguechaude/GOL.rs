use bevy::prelude::{Resource, ResMut};
use bevy_egui::egui;
use gol_config::SimulationConfig;

#[derive(Resource, Default)]
pub struct PlacementMode {
    pub active: bool,
    pub pattern_name: Option<String>,
}

#[derive(Resource, Default)]
pub struct RleLoader {
    pub rle_content: String,
    pub show_input: bool,
    pub error_message: Option<String>,
}

pub fn pattern_system(
    ui: &mut egui::Ui,
    placement_mode: &mut ResMut<PlacementMode>,
    simulation_config: &mut ResMut<SimulationConfig>,
    rle_loader: &mut ResMut<RleLoader>,
) {
    ui.separator();
    ui.vertical(|ui| {
        ui.label("Patterns:");
        ui.horizontal_wrapped(|ui| {
            if ui.button("pulsar").clicked() {
                placement_mode.active = true;
                placement_mode.pattern_name = Some("pulsar".to_string());
                simulation_config.running = false;
            }
            if ui.button("pufferfish").clicked() {
                placement_mode.active = true;
                placement_mode.pattern_name = Some("pufferfish".to_string());
                simulation_config.running = false;
            }
            if ui.button("traffic-jam").clicked() {
                placement_mode.active = true;
                placement_mode.pattern_name = Some("traffic-jam".to_string());
                simulation_config.running = false;
            }
            if ui.button("Load RLE").clicked() {
                rle_loader.show_input = true;
                rle_loader.rle_content.clear();
                rle_loader.error_message = None;
            }
        });

        if placement_mode.active {
            ui.colored_label(
                egui::Color32::GREEN,
                format!(
                    "Click to place: {}",
                    placement_mode.pattern_name.as_ref().unwrap()
                ),
            );
            if ui.button("Cancel").clicked() {
                placement_mode.active = false;
            }
        }
    });
}

pub fn rle_loader_modal(
    ctx: &egui::Context,
    rle_loader: &mut ResMut<RleLoader>,
    placement_mode: &mut ResMut<PlacementMode>,
    simulation_config: &mut ResMut<SimulationConfig>,
) {
    if !rle_loader.show_input {
        return;
    }

    // Background semi transparent when popup appear
    egui::Area::new(egui::Id::new("rle_overlay"))
        .fixed_pos(egui::Pos2::ZERO)
        .show(ctx, |ui| {
            let screen_rect = ctx.input(|i| i.screen_rect);
            ui.allocate_response(screen_rect.size(), egui::Sense::click());
            ui.painter().rect_filled(
                screen_rect,
                egui::CornerRadius::ZERO,
                egui::Color32::from_rgba_premultiplied(0, 0, 0, 150),
            );
        });

    egui::Window::new("Load RLE Pattern")
        .collapsible(false)
        .resizable(true)
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .default_size([500.0, 400.0])
        .max_height(ctx.screen_rect().height() * 0.8)
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.label("Paste your RLE pattern content:");
                ui.add_space(10.0);

                // ScrollArea pour gérer le contenu trop grand
                egui::ScrollArea::vertical()
                    .max_height(250.0)
                    .show(ui, |ui| {
                        let text_edit = egui::TextEdit::multiline(&mut rle_loader.rle_content)
                            .desired_width(f32::INFINITY)
                            .desired_rows(12)
                            .hint_text("Example: bo$2bo$3o!");

                        ui.add(text_edit);
                    });

                if let Some(error) = &rle_loader.error_message {
                    ui.add_space(5.0);
                    ui.colored_label(egui::Color32::RED, error);
                }

                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        rle_loader.show_input = false;
                        rle_loader.rle_content.clear();
                        rle_loader.error_message = None;
                    }

                    ui.add_space(10.0);

                    let load_btn = egui::Button::new("Load Pattern")
                        .fill(egui::Color32::from_rgb(50, 100, 180));

                    if ui.add(load_btn).clicked() {
                        if rle_loader.rle_content.trim().is_empty() {
                            rle_loader.error_message = Some("Please enter RLE content".to_string());
                        } else {
                            // Validate RLE format (basic check)
                            if rle_loader.rle_content.contains('!') {
                                // Close modal and activate placement mode
                                rle_loader.show_input = false;
                                rle_loader.error_message = None;
                                placement_mode.active = true;
                                placement_mode.pattern_name = Some("custom_rle".to_string());
                                simulation_config.running = false;
                            } else {
                                rle_loader.error_message =
                                    Some("Invalid RLE format (missing '!') dumbass !".to_string());
                            }
                        }
                    }
                });
            });
        });
}
