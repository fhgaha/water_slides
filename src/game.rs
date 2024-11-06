mod my_camera;
mod pipe;
mod tube_segment;
mod my_cursor;

use std::ops::DerefMut;

use bevy::{
    color::palettes::css::*, pbr::NotShadowCaster, prelude::*, render::{
        mesh::{Indices, PrimitiveTopology},
        render_asset::RenderAssetUsages,
    }, window::WindowResolution
};
use bevy_mod_raycast::prelude::*;
use bevy_panorbit_camera::*;
use bevy_rts_camera::*;
use my_cursor::MyCursorPlugin;
use tube_segment::TubeSegmentPlugin;
use pipe::PipePlugin;
use crate::my_ui::MyUiPlugin;
use crate::fps::FpsPlugin;

pub struct GamePlugin;

#[derive(Component)]
pub struct ControlPointsPlane;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                DefaultPlugins.set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(1500., 600.),
                        position: WindowPosition::At(IVec2::ZERO),
                        ..default()
                    }),
                    ..default()
                }),
                PanOrbitCameraPlugin,
                // TubeSegmentPlugin,
                PipePlugin,
                MyUiPlugin,
                FpsPlugin,
                MyCursorPlugin
            ))
            .add_systems(
                Startup,
                (
                    setup,
                    // draw_quad
                    // setup_control_points_plane
                ),
            )
            .add_systems(
                Update,
                (
                    // check_quad_normals_system
                    draw_zero_point_gizmos,
                    rotate_control_points_plane,
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

    // Light
    //     DirectionalLight
    commands.spawn((
        Name::new("Directional Light"),
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 10_000.,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_rotation(Quat::from_euler(
                EulerRot::YXZ,
                f32::to_radians(150.),
                f32::to_radians(-40. - 90.),
                0.,
            )),
            ..default()
        })
    );

    //     PointLight
    // commands.spawn((
    //     Name::new("Point Light"),
    //     PointLightBundle{
    //         point_light: PointLight {
    //             color: DARK_CYAN.into(),
    //             ..default()
    //         },
    //         ..default()
    //     },
    // ));

    //     SpotLight
    // commands.spawn((
    //     Name::new("Spot Light Bundle"),
    //     SpotLightBundle {
    //         spot_light: SpotLight {
    //             color: LIGHT_BLUE.into(),
    //             intensity: 100_000_000., 
    //             ..default()
    //         },
    //         ..default()
    //     }
    // ));

    //camera
    commands.spawn((
        Camera3dBundle::default(), 
        PanOrbitCamera::my_setup()
    ))
    .with_children(|cam|{
        cam.spawn((
            Name::new("Spot Light Bundle"),
            SpotLightBundle {
                spot_light: SpotLight {
                    color: LIGHT_BLUE.into(),
                    ..default()
                },
                ..default()
            }
        ));
    });
 
    // cube
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
    //     material: materials.add(Color::srgb_u8(124, 144, 255)),
    //     transform: Transform::from_xyz(0.0, 0.5, 0.0),
    //     ..default()
    // });
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
        Vec3::new( 1., -1., 0.),
        Vec3::new(-1., -1., 0.),
    ];

    //0 ._____. 1
    //  |    /|
    //  |  /  |
    //3 ./____. 2
    //counter-clock-wise
    let tri_indices = vec![0, 3, 1, 1, 3, 2];

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

fn setup_control_points_plane(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
){
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
    // mut gizmos: Gizmos,
) { 
    let _plane_pos = Vec3::ZERO;
    let _plane_size = Vec2::splat(40.);

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
}
