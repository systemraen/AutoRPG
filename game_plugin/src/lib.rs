use bevy::prelude::{App, Plugin};

mod ui_plugin;

#[derive(Default)]
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(ui_plugin::UiPlugin);
    }
}
