mod ui_system;

use bevy::{app::PluginGroupBuilder, prelude::PluginGroup};

#[derive(Default)]
struct GamePlugins {
    game_state: GameState,
}

impl PluginGroup for GamePlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {}
}

#[derive(Default)]
struct GameState {
    click_count: i32,
}
