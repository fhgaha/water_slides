use std::array;

use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, PrimitiveTopology, RectangleMeshBuilder},
        render_asset::RenderAssetUsages,
    },
    window::WindowResolution,
};
use bevy_mod_raycast::prelude::*;
use bevy_rts_camera::*;

pub struct GamePlugin;

#[derive(Component)]
struct Cursor;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(800., 600.),
                    position: WindowPosition::At(IVec2::ZERO),
                    ..default()
                }),
                ..default()
            }),
            RtsCameraPlugin,
        ))
        .add_systems(Startup, (setup, setup_cursor, setup_verts))
        .add_systems(Update, draw_cursor);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(80.0, 80.0)),
            material: materials.add(Color::linear_rgba(0.3, 0.5, 0., 0.0)),
            ..default()
        },
        // Add `Ground` component to any entity you want the camera to treat as ground.
        Ground,
    ));

    // Light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 1000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::YXZ,
            150.0f32.to_radians(),
            -40.0f32.to_radians(),
            0.0,
        )),
        ..default()
    });

    // Camera
    commands.spawn((
        Camera3dBundle::default(),
        RtsCamera::default(),
        RtsCameraControls::my_controls(),
    ));
}

fn setup_cursor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere::new(1.)),
            material: materials.add(Color::srgb(1., 1., 1.)),
            transform: Transform::from_xyz(0.0, 0.0, -23.0),
            ..default()
        },
        Cursor,
    ));
}

fn draw_cursor(
    cameras: Query<(&Camera, &GlobalTransform)>,
    ground_q: Query<&GlobalTransform, With<Ground>>,
    windows: Query<&Window>,
    mut cursors: Query<&mut Transform, With<Cursor>>,
    // mut gizmos: Gizmos,
    mut raycast: Raycast,
) {
    let (camera, camera_transform) = cameras.single();
    // let ground = ground_q.single();
    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };
    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };
    // let intersections = raycast.debug_cast_ray(
    //     ray,
    //     &RaycastSettings {
    //         filter: &|e| ground_q.contains(e),
    //         ..default()
    //     },
    //     &mut gizmos,
    // );
    let intersections = raycast.cast_ray(
        ray,
        &RaycastSettings {
            filter: &|e| ground_q.contains(e),
            ..default()
        },
    );
    let point: Vec3 = match intersections.len() > 0 {
        true => intersections[0].1.position(),
        false => Vec3::ZERO,
    };

    //Gizmo cirle
    // gizmos
    //     .circle(point + ground.up() * 0.01, ground.up(), 0.2, Color::WHITE)
    //     .resolution(12);

    //Gizmo sphere
    // gizmos
    //     .sphere(point, default(), 0.2, Color::WHITE)
    //     .resolution(8);

    for mut c in cursors.iter_mut() {
        c.translation = point;
        println!("pt: {}", point);
    }
}

fn setup_verts(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let points = vec![
        Vec3::new(-1., 1., 0.),
        Vec3::new(1., 1., 0.),
        Vec3::new(-1., -1., 0.),
        Vec3::new(1., -1., 0.),
    ];

    let tri_indices = vec![2, 1, 0, 1, 2, 3];

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, points);
    mesh.insert_indices(Indices::U32(tri_indices));

    let mesh_handle = meshes.add(mesh);

    commands.spawn(PbrBundle {
        mesh: mesh_handle,
        material: materials.add(StandardMaterial {
            base_color: Color::WHITE,
            ..Default::default()
        }),
        ..Default::default()
    });
}
