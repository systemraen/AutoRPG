use bevy::prelude::{App, Plugin, ResMut};
use bevy_egui::{egui, EguiContext};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(ui_system);
    }
}

fn ui_system(mut egui_ctx: ResMut<EguiContext>) {
    egui::CentralPanel::default().show(egui_ctx.ctx_mut(), |_ui| {

    });
}
