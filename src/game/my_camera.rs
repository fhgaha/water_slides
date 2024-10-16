use bevy::{math::VectorSpace, prelude::*};

pub struct MyCameraPlugin;

impl Plugin for MyCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, update_position);
    }
}

#[derive(Component)]
struct MyCamTacker;

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0., 20., 16.).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        MyCamTacker,
    ));
}

fn update_position(
    button_input: Res<ButtonInput<KeyCode>>,
    mut cam_transforms: Query<&mut Transform, With<MyCamTacker>>,
) {
    let mut delta = Vec3::ZERO;

    let mut cam_trm = cam_transforms.single_mut();

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