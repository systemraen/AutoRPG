use bevy::prelude::ResMut;
use bevy_egui::{EguiContext, egui::CentralPanel};

struct UiSystem {
    
}

fn ui_system(mut egui_ctx: ResMut<EguiContext>) {
    CentralPanel::default().show(egui_ctx.ctx_mut(), |ui| {
        add_clicker(ui);
    });
}

fn add_clicker(ui: &mut Ui) {
    ui.with_layout(
        Layout::from_main_dir_and_cross_align(egui::Direction::TopDown, egui::Align::Center),
        |ui| {
            if ui
                .add_sized(
                    [200.0, 200.0],
                    Button::new(RichText::new("Click me!").size(32.0)),
                )
                .clicked()
            {
                ui_state.click_count += 1;
            }

            ui.label(format!("You've clicked {} times!", ui_state.click_count));
        },
    );
}