use bevy::{prelude::{App, Plugin, KeyCode, Commands, Res}, input::Input};
use bevy_egui::EguiPlugin;
use iyes_loopless::prelude::*;

mod ui_plugin;

#[derive(Default)]
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GameContext>()
            .add_plugin(EguiPlugin)  
            .add_plugin(ui_plugin::UiPlugin)
            .add_loopless_state(GameState::TitleScreen)
            .add_system(leave_title.run_in_state(GameState::TitleScreen));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    TitleScreen, TitleMenu, LoadMenu, SaveMenu, PlayGame, GamePause
}

#[derive(Default)]
pub struct GameContext;

/// Transition back to menu on pressing Escape
fn leave_title(mut commands: Commands, kbd: Res<Input<KeyCode>>) {
    if kbd.just_pressed(KeyCode::Space) {
        commands.insert_resource(NextState(GameState::PlayGame));
    }
}
