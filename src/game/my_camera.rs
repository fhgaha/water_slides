use std::f32::consts::{PI, TAU};

use bevy::{input::mouse::MouseMotion, prelude::*, window::PrimaryWindow};

pub struct MyCameraPlugin;

impl Plugin for MyCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, 
                (
                    update_position,
                    update_rotation,
                    update_zoom
                )
            );
    }
}

#[derive(Component)]
pub struct MyCamTacker;

fn setup(mut commands: Commands) {
    // commands.spawn((
    //     Camera3dBundle {
    //         transform: Transform::from_xyz(0., 20., 16.).looking_at(Vec3::ZERO, Vec3::Y),
    //         ..default()
    //     },
    //     MyCamTacker,
    // ));
}

//  cam_trm.forward() - moves cam to the looking at
//  cam_trm.back()    - moves cam away from looking at

fn update_position(
    button_input: Res<ButtonInput<KeyCode>>,
    mut cam_transforms: Query<&mut Transform, With<MyCamTacker>>,
) {
    let mut delta = Vec3::ZERO;

    let mut cam_trm = match cam_transforms.get_single_mut() {
        Ok(cam_transform) => cam_transform,
        Err(_) => return,
    };

    // Keyboard pan
    if button_input.pressed(KeyCode::KeyW) {
        delta += project_onto_xz(cam_trm.forward().as_vec3()).normalize()
    }
    if button_input.pressed(KeyCode::KeyS) {
        delta += project_onto_xz(cam_trm.back().as_vec3()).normalize()
    }
    if button_input.pressed(KeyCode::KeyA) {
        delta += Vec3::from(cam_trm.left())
    }
    if button_input.pressed(KeyCode::KeyD) {
        delta += Vec3::from(cam_trm.right())
    }

	cam_trm.translation += delta;
}

fn project_onto_xz(v: Vec3) -> Vec3 {
    let normal = Vec3::Y; // Normal for XZ plane
    let projection = v - (v.dot(normal) / normal.length_squared()) * normal;
    projection
}

fn update_rotation(
    button_input: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut cam_transforms: Query<&mut Transform, With<MyCamTacker>>,
    mut primary_window_q: Query<&mut Window, With<PrimaryWindow>>,
) {
    let Ok(mut primary_window) = primary_window_q.get_single_mut() else {return;};

    let mut cam_trm = cam_transforms.single_mut();

    if button_input.pressed(MouseButton::Right) {
        let mouse_delta: Vec2 = mouse_motion.read().map(|e| e.delta).sum();

        //hor rot

        // Adjust based on window size, so that moving mouse entire width of window
        // will be one half rotation (180 degrees)
        let delta_x = mouse_delta.x / primary_window.width() * PI;

        // cam_trm.rotate_around(Vec3::ZERO, Quat::from_rotation_y(-delta_x));


        let delta_y = mouse_delta.y / 100.;
        cam_trm.rotate_around(Vec3::ZERO, Quat::from_axis_angle(Vec3::Y, delta_y));


    }
}

fn update_zoom(
    // button_input: Res<ButtonInput<MouseButton>>,
    // mut mouse_motion: EventReader<MouseMotion>,
    // mut cam_transforms: Query<&mut Transform, With<MyCamTacker>>,
    // mut primary_window_q: Query<&mut Window, With<PrimaryWindow>>,
) {
    
}