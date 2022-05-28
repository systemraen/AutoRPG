use crate::{GameContext, GameState};
use bevy::prelude::{App, Plugin, ResMut};
use bevy_egui::{
    egui::{self, Align2},
    EguiContext,
};
use iyes_loopless::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(title_screen.run_in_state(GameState::TitleScreen))
            .add_system(game_ui.run_in_state(GameState::PlayGame));
    }
}

struct Bar {
    buttons: Vec<BarButton>,
}

struct BarButton {
    /*
    image, width, height, frame, background
    */
    action: Action,
}

struct Action;

static BUTTON_SIZE: f32 = 42.0;

fn game_ui(mut egui_ctx: ResMut<EguiContext>, _game_ctx: ResMut<GameContext>) {
    let egui_ctx = egui_ctx.ctx_mut();
    egui::Window::new("Action Bar") // TODO pull in settings for bar
        .anchor(Align2::CENTER_BOTTOM, (0., 0.))
        .title_bar(false)
        .resizable(false)
        //.frame(egui::Frame)
        .show(egui_ctx, |action_ui| {
            if action_ui
                .add_sized([BUTTON_SIZE, BUTTON_SIZE], egui::Button::new(""))
                .clicked()
            {}
        });
}

fn title_screen(mut egui_ctx: ResMut<EguiContext>) {
    egui::Window::new("")
        .fixed_pos((500., 500.))
        .title_bar(false)
        .resizable(false)
        .frame(egui::Frame::none())
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.heading("AutoRPG");
        });
}
