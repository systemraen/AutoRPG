use bevy::prelude::{DefaultPlugins, WindowDescriptor, default, App};
use bevy_egui::EguiPlugin;
use game_plugin::GamePlugin;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "AutoRPG".to_string(),
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .add_plugin(EguiPlugin)        
        .run();
}
