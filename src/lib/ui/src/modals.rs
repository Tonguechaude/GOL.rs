//! # Modals Module
//!
//! Modal dialogs for confirmation and input.

use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

/// State for managing modal windows
#[derive(Default, Resource)]
pub struct ModalState {
    pub show_reset: bool,
    pub show_random: bool,
}

/// Plugin for modal dialog systems
pub struct ModalsPlugin;

impl Plugin for ModalsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ModalState>()
            .add_systems(bevy_egui::EguiPrimaryContextPass, modal_system);
    }
}

/// System that handles modal dialog rendering and interaction
pub fn modal_system(mut contexts: EguiContexts, mut modal_state: ResMut<ModalState>) {
    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };

    // Reset confirmation modal
    if modal_state.show_reset {
        render_overlay(ctx);

        egui::Window::new("⚠ Kill all cells!")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(10.0);
                    ui.label("Are you sure you want to kill all cells?");
                    ui.add_space(15.0);

                    ui.horizontal(|ui| {
                        ui.spacing_mut().button_padding = egui::Vec2::new(20.0, 10.0);

                        if ui.button("No").clicked() {
                            modal_state.show_reset = false;
                        }

                        ui.add_space(10.0);

                        let delete_btn =
                            egui::Button::new("Yes").fill(egui::Color32::from_rgb(180, 50, 50));
                        if ui.add(delete_btn).clicked() {
                            modal_state.show_reset = false;
                        }
                    });
                    ui.add_space(5.0);
                });
            });
    }

    // Random generation modal
    if modal_state.show_random {
        render_overlay(ctx);

        egui::Window::new("Random Generation")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(10.0);
                    ui.label("Fill the grid with random cells?");
                    ui.add_space(5.0);
                    ui.label("Grid size: 50×50"); // TODO: Get from config
                    ui.add_space(15.0);

                    ui.horizontal(|ui| {
                        ui.spacing_mut().button_padding = egui::Vec2::new(20.0, 10.0);

                        if ui.button("No").clicked() {
                            modal_state.show_random = false;
                        }

                        ui.add_space(10.0);

                        let generate_btn =
                            egui::Button::new("Yes").fill(egui::Color32::from_rgb(50, 100, 180));
                        if ui.add(generate_btn).clicked() {
                            modal_state.show_random = false;
                            // The actual generation will be handled by the controls module
                        }
                    });
                    ui.add_space(5.0);
                });
            });
    }
}

/// Renders a semi-transparent overlay behind modals
fn render_overlay(ctx: &egui::Context) {
    egui::Area::new(egui::Id::new("modal_overlay"))
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
}
