use bevy::prelude::{App, Plugin, ResMut};
use bevy_egui::{
    egui::{self, Align2},
    EguiContext,
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(ui_system);
    }
}

fn ui_system(mut egui_ctx: ResMut<EguiContext>, game_state: ResMut<crate::GameState>) {
    egui::Window::new("")
        .anchor(Align2::CENTER_CENTER, (0., 0.))
        .title_bar(false)
        .resizable(false)
        .collapsible(false)
        .show(egui_ctx.ctx_mut(), |ui| {
            add_clicker(ui, game_state);
        });
}

fn add_clicker(ui: &mut egui::Ui, mut game_state: ResMut<crate::GameState>) {
    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
        if ui
            .add_sized(
                [200.0, 100.0],
                egui::Button::new(egui::RichText::new("Click me!").size(32.0)),
            )
            .clicked()
        {
            game_state.click_count += 1;
        }

        let click_text = format!("You've clicked {} times!", game_state.click_count);
        ui.label(egui::RichText::new(click_text).size(32.0));
    });
}
