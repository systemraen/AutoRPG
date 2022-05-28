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

pub enum GameContext {
    TitleScreen, TitleMenu, LoadMenu, SaveMenu, Game, GamePause
}

pub struct GameState {
    pub game_context: GameContext
}

impl Default for GameState {
    fn default() -> Self {
        Self { game_context: GameContext::Game }
    }
}
