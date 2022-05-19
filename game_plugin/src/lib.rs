use bevy::prelude::{App, Plugin};

mod ui_system;

#[derive(Default)]
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GameState>()
            .add_system(ui_system::ui_system);
    }
}

#[derive(Default)]
pub struct GameState {
    click_count: i32,
}
