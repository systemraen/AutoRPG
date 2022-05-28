use crate::{GameContext, GameState};
use bevy::prelude::{App, Plugin, ResMut};
use bevy_egui::{
    egui::{self, Align2},
    EguiContext,
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(ui_system);
    }
}

fn ui_system(mut egui_ctx: ResMut<EguiContext>, game_state: ResMut<GameState>) {
    let egui_ctx = egui_ctx.ctx_mut();
    match game_state.game_context {
        GameContext::TitleScreen => todo!(),
        GameContext::TitleMenu => todo!(),
        GameContext::LoadMenu => todo!(),
        GameContext::SaveMenu => todo!(),
        GameContext::Game => make_game_ui(egui_ctx, game_state),
        GameContext::GamePause => todo!(),
    };
}

struct Bar {
    buttons: Vec<BarButton>
}

struct BarButton {
    /*
    image, width, height, frame, background
    */

    action: Action
}

struct Action;

static BUTTON_SIZE: f32 = 42.0;

fn make_game_ui(mut egui_ctx: &egui::Context, _game_state: ResMut<GameState>) {
    egui::Window::new("Action Bar") // TODO pull in settings for bar
        .anchor(Align2::CENTER_BOTTOM, (0., 0.))
        .title_bar(false)
        .resizable(false)
        //.frame(egui::Frame)
        .show(egui_ctx, |action_ui| {
            if action_ui.add_sized([BUTTON_SIZE, BUTTON_SIZE], egui::Button::new("")).clicked() {
                
            }
        });
}
