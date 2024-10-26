use bevy::prelude::*;
use bevy_egui::*;

pub struct MyUiPlugin;

impl Plugin for MyUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<UiState>()
            .add_plugins(EguiPlugin)
            .add_systems(Update, ui_example_system);
    }
}

#[derive(Debug, Default, Resource)]
pub struct UiState {
    pub value: f32,
}

fn ui_example_system(
    mut contexts: EguiContexts,
    mut ui_state: ResMut<UiState>,
) {
    egui::Window::new("Hello").show(
        contexts.ctx_mut(), 
        |ui| {
            ui.add(egui::Slider::new(&mut ui_state.value, 0.0..=1.0).text("t value"));
        }
    );
}
                    
