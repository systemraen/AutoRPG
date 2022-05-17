use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

fn main() {
    App::new()
        .insert_resource(GameState::default())
        .insert_resource(WindowDescriptor {
            title: "AutoRPG".to_string(),
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_system(ui_system)
        .run();
}

#[derive(Default)]
struct GameState {
    click_count: i32,
}

fn ui_system(mut egui_ctx: ResMut<EguiContext>, mut ui_state: ResMut<GameState>) {
    egui::CentralPanel::default().show(egui_ctx.ctx_mut(), |ui| {
        if ui.button("Click me").clicked() {
            ui_state.click_count += 1;
        }
        ui.label(format!("You've clicked {} times!", ui_state.click_count));
    });
}
