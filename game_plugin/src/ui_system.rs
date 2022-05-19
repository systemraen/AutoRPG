use bevy::prelude::{ResMut};
use bevy_egui::{EguiContext, egui};

pub fn ui_system(mut egui_ctx: ResMut<EguiContext>, game_state: ResMut<crate::GameState>) {
    egui::CentralPanel::default().show(egui_ctx.ctx_mut(), |ui| {
        add_clicker(ui, game_state);
    });
}

fn add_clicker(ui: &mut egui::Ui, mut game_state: ResMut<crate::GameState>) {
    ui.with_layout(
        egui::Layout::from_main_dir_and_cross_align(egui::Direction::TopDown, egui::Align::Center),
        |ui| {
            if ui
                .add_sized(
                    [200.0, 200.0],
                    egui::Button::new(egui::RichText::new("Click me!").size(32.0)),
                )
                .clicked()
            {
                game_state.click_count += 1;
            }

            ui.label(format!("You've clicked {} times!", game_state.click_count));
        },
    );
}