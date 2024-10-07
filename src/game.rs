
use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, PrimitiveTopology},
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
        .add_systems(
            Startup,
            (
                setup,
                setup_cursor,
                //draw_quad
                setup_control_points,
            ),
        )
        .add_systems(
            Update,
            (
                draw_cursor,
                // check_quad_normals_system
                (update_control_point_state, update_control_points_positions).chain(),
            ),
        );
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

    //Test light
    // commands.spawn(PointLightBundle {
    //     point_light: PointLight {
    //         intensity: 100_000.,
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     transform: Transform::from_rotation(Quat::from_euler(EulerRot::YXZ, 0., 0., 1.)),

    //     ..default()
    // });

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
    mut gizmos: Gizmos,
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
    //     .sphere(point, default(), 1., Color::WHITE)
    //     .resolution(8);

    for mut c in cursors.iter_mut() {
        c.translation = point;
    }
}

fn draw_quad(
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
    mesh.compute_normals();

    let mesh_handle = meshes.add(mesh);

    commands.spawn(PbrBundle {
        mesh: mesh_handle,
        material: materials.add(StandardMaterial {
            base_color: Color::WHITE,
            ..default()
        }),
        ..default()
    });
}

fn check_quad_normals_system(
    mut lights: Query<&mut Transform, With<PointLight>>,
    time: Res<Time>,
    mut gismos: Gizmos,
) {
    for mut transform in lights.iter_mut() {
        let mut new_translation = Vec3::new(
            transform.translation.x,
            transform.translation.y,
            transform.translation.z,
        );
        new_translation.z = (time.elapsed_seconds() * 10.).sin();
        transform.translation = new_translation;

        gismos
            .sphere(new_translation, default(), 0.2, Color::WHITE)
            .resolution(8);
    }
}

#[derive(Component)]
struct RoadSegment {
    control_points: [Transform; 4],
}

impl RoadSegment {
    pub fn get_pos(&self, i: usize) -> Vec3 {
        self.control_points[i].translation
    }
}

enum ControlPointState {
    None,
    Drag,
}

#[derive(Component)]
struct ControlPoint {
    pub state: ControlPointState,
}

fn setup_control_points(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let trfrms = vec![
        Transform::from_xyz(-10., 0., 10.),
        Transform::from_xyz(-10., 0., -10.),
        Transform::from_xyz(10., 0., 10.),
        Transform::from_xyz(10., 0., -10.),
    ];

    for t in trfrms {
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Sphere::new(1.)),
                material: materials.add(Color::srgb(1., 1., 1.)),
                transform: t,
                ..default()
            },
            ControlPoint {
                state: ControlPointState::None,
            },
        ));
    }
}

fn update_control_point_state(
    cameras: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut raycast: Raycast,
    mut control_points: Query<&mut ControlPoint>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    let (camera, camera_transform) = cameras.single();

    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };
    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    let intersections = raycast.cast_ray(
        ray,
        &RaycastSettings {
            filter: &|e| control_points.contains(e),
            ..default()
        },
    );
    if intersections.len() > 0 {
        if let Ok(mut ctrl_pt) = control_points.get_mut(intersections[0].0) {
            if buttons.pressed(MouseButton::Left) {
                ctrl_pt.state = ControlPointState::Drag;
            } else {
                ctrl_pt.state = ControlPointState::None;
            }
        }
    }
}

fn update_control_points_positions(
    cursors: Query<&Transform, (With<Cursor>, Without<ControlPoint>)>,
    mut ctrl_pts_transforms: Query<(&mut Transform, &ControlPoint)>,
) {
    let cursor = cursors.single();

    for (mut t, ctrl_pt) in ctrl_pts_transforms.iter_mut() {
        if let ControlPointState::Drag = ctrl_pt.state {
            t.translation = cursor.translation
        }
    }
}
