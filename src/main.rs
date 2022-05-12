use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {            
            title: "AutoRPG".to_string(),
            ..default()         
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        //.add_startup_system(setup)
        .add_system(ui_example)
        .run();
}

//fn _setup(mut commands: Commands, asset_server: Res<AssetServer>) {}

fn ui_example(mut egui_ctx: ResMut<EguiContext>) {
    egui::CentralPanel::default().show(egui_ctx.ctx_mut(), |ui| {
        if ui.button("Click me").clicked() {
            
        }
    });
}
