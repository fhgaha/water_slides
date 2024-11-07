use bevy::prelude::*;
use bevy_mod_raycast::prelude::*;

use super::my_cursor::{self, MyCursor, MyCursorData};

pub struct PipePlugin;

#[derive(Component)]
pub struct BarEdge;

#[derive(Component)]
pub struct Pipe{
    pub list: Vec<Transform>
}

impl Plugin for PipePlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, setup);
        app.add_systems(Update, update);
	}
}

fn setup(
	mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
){
	commands.spawn((
        Name::new("Polygon surface"),
        BarEdge,
        PbrBundle{
            mesh: meshes.add(RegularPolygon::default()),
            transform: Transform::from_xyz(5., 17., 0.),
            ..default()
        }
    ));
}

fn update(
    res_cursor_data: Res<MyCursorData>,
    cursors: Query<(&Transform, &MyCursor)>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    bar_edges: Query<&BarEdge>,
){
    let data = res_cursor_data.into_inner();
    if let Some((ent, intersection_data)) = &data.intersection {
        if bar_edges.contains(*ent) {
            for (cursor_trm, my_cursor) in cursors.iter() {
                if mouse_buttons.pressed(MouseButton::Left) {
                    
                    let dist: f32 = 1.;

                    let trm = *cursor_trm;

                    let new_transl = trm.translation + -trm.forward() * dist;

                    let new_trm = Transform::from_translation(new_transl);

                    commands.spawn((
                        Name::new("Polygon surface"),
                        BarEdge,
                        PbrBundle{
                            mesh: meshes.add(RegularPolygon::default()),
                            material: materials.add(StandardMaterial::default()),
                            transform: new_trm,
                            ..default()
                        }
                    ));

                }
            }   
        }
    }
}