use bevy::{prelude::*, transform::commands};
use bevy_mod_raycast::prelude::*;
use bevy_rts_camera::Ground;
use crate::game::pipe;


pub struct MyCursorPlugin;

#[derive(Component)]
pub struct MyCursor;

#[derive(Resource)]
pub struct MyCursorData {
    pub intersection: Option<(Entity, IntersectionData)>
}

impl Plugin for MyCursorPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(MyCursorData{ intersection: None });
		app.add_systems(Startup, setup);
		app.add_systems(Update, (
            update_intersection,
            draw
        ));
	}
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere::new(1.)),
            material: materials.add(Color::srgb(1., 1., 1.)),
            transform: Transform::from_xyz(0.0, 0.0, -23.0),
            visibility: Visibility::Hidden,
            ..default()
        },
        MyCursor,
    ));
}

fn update_intersection(
    cameras: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    cursor_interactables: 
        Query<
            &Transform, (Or<(
                    With<Ground>, 
                    // With<ControlPointDraggable>
                    With<pipe::BarEdge>
                )>, 
                Without<MyCursor>
            )
        >,
    mut raycast: Raycast,
	mut res_my_cursor_intersection: ResMut<MyCursorData>,
) {
    let (camera, camera_transform) = cameras.single();
    let Some(cursor_position) = windows.single().cursor_position() else {return;};
    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {return;};

    //raycast to control points plane
    let intersections = raycast.cast_ray(
        ray,
        &RaycastSettings {
            filter: &|e| cursor_interactables.contains(e),
            ..default()
        },
    );

    //register in resourse
	res_my_cursor_intersection.intersection = match intersections.len() > 0 {
		true => Some(intersections[0].clone()),
		false => None,
	};
}

fn draw(
    mut res_my_cursor_intersection: ResMut<MyCursorData>,
    mut cursor_transforms: Query<&mut Transform, With<MyCursor>>,
    mut gizmos: Gizmos,
){
    let point: Vec3 = match &res_my_cursor_intersection.intersection {
        Some(val) => val.1.position(),
        None => Vec3::ZERO,
    };

    for mut cursor_trm in cursor_transforms.iter_mut() {
        cursor_trm.translation = point;
    }

    //Gizmo sphere
    gizmos
        .sphere(point, default(), 1., Color::WHITE)
        .resolution(8);
}