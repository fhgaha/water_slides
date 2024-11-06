use bevy::prelude::*;
use bevy_mod_raycast::prelude::*;

pub struct PipePlugin;

#[derive(Component)]
pub struct BarEdge;

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
	// cameras: Query<(&Camera, &GlobalTransform)>,
	// windows: Query<&Window>,
    // mut cursor_transforms: Query<&mut Transform, With<MyCursor>>,
    // bar_edges: Query<&Transform, (With<BarEdge>, Without<MyCursor>)>,
	// mut raycast: Raycast,
    // mut gizmos: Gizmos,
){
	// let (camera, camera_transform) = cameras.single();
    // let Some(cursor_position) = windows.single().cursor_position() else {return;};
    // let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {return;};

    // //raycast to control points plane
    // let intersections = raycast.cast_ray(
    //     ray,
    //     &RaycastSettings {
    //         filter: &|e| bar_edges.contains(e),
    //         ..default()
    //     },
    // );

    // let point: Vec3 = match intersections.len() > 0 {
    //     true => intersections[0].1.position(),
    //     false => Vec3::ZERO,
    // };


    // for mut cursor_trm in cursor_transforms.iter_mut() {
    //     cursor_trm.translation = point;
    // }

    // //Gizmo sphere
    // gizmos
    //     .sphere(point, default(), 1., Color::WHITE)
    //     .resolution(8);
}