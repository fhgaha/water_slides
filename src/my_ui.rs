use bevy::prelude::*;
use bevy_egui::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct MyUiPlugin;

impl Plugin for MyUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(UiState { t_value: 0., sections_amnt: 8 })
            .add_plugins(WorldInspectorPlugin::new())
            //conflicts with inspector
            // .add_plugins(EguiPlugin)
            .add_systems(Update, read_slider_value);
    }
}

#[derive(Debug, Default, Resource)]
pub struct UiState {
    pub t_value: f32,
    pub sections_amnt: i32
}

fn read_slider_value(
    mut contexts: EguiContexts,
    mut ui_state: ResMut<UiState>,
) {
    egui::Window::new("Hello").show(
        contexts.ctx_mut(), 
        |ui| {
            ui.add(egui::Slider::new(&mut ui_state.t_value, 0.0..=1.0)
                .text("t value"));
            ui.add(egui::Slider::new(&mut ui_state.sections_amnt, 2..=120)
                .text("Sections amnt"));
            ui.separator();
            // ui.add(egui::Label::new("CP1 pos:"));
            // ui.add(egui::Label::new("x:"));
            // ui.add(egui::Label::new("y:"));
            // ui.add(egui::Label::new("z:"));
            
        }
    );
}
                    
