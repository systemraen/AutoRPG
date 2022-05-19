use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Button, CentralPanel, Layout, RichText, Ui},
    EguiContext, EguiPlugin,
};

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "AutoRPG".to_string(),
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(Game::default())
        .run();
}
