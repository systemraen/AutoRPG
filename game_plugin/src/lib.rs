use bevy::prelude::{App, Plugin};

mod ui_plugin;

#[derive(Default)]
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GameState>()
            .add_plugin(ui_plugin::UiPlugin);
    }
}

#[derive(Default)]
pub struct GameState {
    click_count: i32,
}
