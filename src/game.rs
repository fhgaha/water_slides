mod my_camera;

use bevy::{
    color::palettes::css::*, pbr::NotShadowCaster, prelude::*, render::{
        mesh::{Indices, PrimitiveTopology},
        render_asset::RenderAssetUsages,
    }, window::WindowResolution
};
use bevy_mod_raycast::prelude::*;
use bevy_panorbit_camera::*;
use bevy_rts_camera::*;

use crate::road_segment::RoadSegmentPlugin;

pub struct GamePlugin;

#[derive(Component)]
pub struct Cursor;

#[derive(Component)]
pub struct ControlPointsPlane;

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
            // RtsCameraPlugin,
            PanOrbitCameraPlugin,
            RoadSegmentPlugin,
        ))
        .add_systems(
            Startup,
            (
                setup,
                setup_cursor,
                //draw_quad
            ),
        )
        .add_systems(
            Update,
            (
                draw_cursor,
                // check_quad_normals_system
                draw_zero_point_gizmos,
                rotate_control_points_plane
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
            mesh: meshes.add(Plane3d::default().mesh().size(80., 80.)),
            material: materials.add(Color::linear_rgba(0.3, 0.5, 0.0, 0.4)),
            // visibility: Visibility::Hidden,
            ..default()
        },
        // Add `Ground` component to any entity you want the camera to treat as ground.
        Ground,
    ));

    // Transparent plane to move control points on
    commands.spawn((
        PbrBundle{
            mesh: meshes.add(Plane3d::default().mesh().size(40., 40.)),
            material: materials.add(
                Color::srgba(0., 0., 1., 0.2),
            ),
            // visibility: Visibility::Hidden,
            ..default()
        },
        NotShadowCaster,
        ControlPointsPlane
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

    commands.spawn((Camera3dBundle::default(), PanOrbitCamera::my_setup()));
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
            visibility: Visibility::Hidden,
            ..default()
        },
        Cursor,
    ));
}


fn draw_cursor(
    cameras: Query<(&Camera, &GlobalTransform)>,
    // control_points_planes: Query<&GlobalTransform, With<ControlPointsPlane>>,
    windows: Query<&Window>,
    mut cursor_transforms: Query<&mut Transform, With<Cursor>>,
    // ctrl_pts_transforms: Query<&Transform, With<ControlPointDraggable>>,
    cursor_interactables: 
        Query<
            &Transform, (
                Or<(
                    With<Ground>, 
                    // With<ControlPointDraggable>
                )>, 
                Without<Cursor>
            )
        >,
    mut raycast: Raycast,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform) = cameras.single();
    // let ground = ground_q.single();
    let Some(cursor_position) = windows.single().cursor_position() else {return;};
    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {return;};

    //raycast to control points plane
    let intersections = raycast.cast_ray(
        ray,
        &RaycastSettings {
            // filter: &|e| control_points_planes.contains(e),
            filter: &|e| cursor_interactables.contains(e),
            ..default()
        },
    );

    let point: Vec3 = match intersections.len() > 0 {
        true => intersections[0].1.position(),
        false => Vec3::ZERO,
    };

    for mut cursor_trm in cursor_transforms.iter_mut() {
        cursor_trm.translation = point;
    }

    //Gizmo sphere
    gizmos
        .sphere(point, default(), 1., Color::WHITE)
        .resolution(8);

}

#[allow(dead_code)]
fn draw_quad(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let points = vec![
        Vec3::new(-1.,  1., 0.),
        Vec3::new( 1.,  1., 0.),
        Vec3::new(-1., -1., 0.),
        Vec3::new( 1., -1., 0.),
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

#[allow(dead_code)]
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

fn draw_zero_point_gizmos(mut gizmos: Gizmos, windows: Query<&Window>) {
    let Ok(window) = windows.get_single() else {return;};

    let length = window.physical_height() as f32;

    //zero
    gizmos.arrow(Vec3::ZERO, Vec3::X * length, RED);
    gizmos.arrow(Vec3::ZERO, Vec3::Y * length, GREEN);
    gizmos.arrow(Vec3::ZERO, Vec3::Z * length, BLUE);
}

fn rotate_control_points_plane(
    mut transforms: ParamSet<(
        Query<&mut Transform, With<ControlPointsPlane>>,            //plane trm
        Query<&Transform, (With<Camera3d>, With<PanOrbitCamera>)>,  //cam trm
    )>,
    mut gizmos: Gizmos,
) { 
    let plane_pos = Vec3::ZERO;
    let plane_size = Vec2::splat(40.);

    let mut temp_trm= Transform::default();

    if let Ok(cam_trm) = transforms.p1().get_single() {
        temp_trm = cam_trm.clone();
    }

    if let Ok(mut plane_trm) = transforms.p0().get_single_mut() {
        plane_trm.look_to(
            temp_trm.local_y(), 
            Vec3::Y
        );
    }

    //doesnt work
    // plane gizmos
    // gizmos.primitive_3d(
    //     &Plane3d {
    //         half_size: plane_size,
    //         ..default()
    //     }, 
    //     temp_trm.translation, 
    //     temp_trm.rotation, 
    //     Color::WHITE
    // );
    // gizmos.rect(temp_trm.translation, temp_trm.rotation, plane_size, Color::WHITE);
}
