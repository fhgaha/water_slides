use std::any::Any;

use bevy::{ecs::{component::ComponentIdFor, observer::TriggerTargets}, log::tracing_subscriber::filter::combinator::And, prelude::*};
use bevy_mod_raycast::prelude::*;

use crate::game::{ControlPointsPlane, Cursor};

pub struct RoadSegmentPlugin;

impl Plugin for RoadSegmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(
            Update,
            ((update_states, update_positions).chain(), draw_spline),
        );
    }
}

#[derive(Bundle, Default)]
struct RoadSegmentBundle {
    transform: Transform,
}

impl RoadSegmentBundle {
    // pub fn get_pos(&self, i: usize) -> Vec3 {
    //     // self.control_points[i]
    //     [self.p0, self.p1, self.p2, self.p3][i].translation
    // }
}

#[derive(PartialEq)]
enum ControlPointState {
    None,
    Drag,
}

#[derive(Component)]
struct ControlPointDraggable {
    pub state: ControlPointState,
}

#[derive(Component)]
#[allow(dead_code)]
struct Curve(CubicCurve<Vec3>);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let positions = [
        Vec3::new(-10., 0.,  10.),
        Vec3::new(-10., 0., -10.),
        Vec3::new( 10., 0., -10.),
        Vec3::new( 10., 0.,  10.),
    ];

    for p in positions {
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Sphere::new(1.)),
                material: materials.add(Color::srgb(1., 1., 1.)),
                transform: Transform::from_xyz(p.x, p.y, p.z),
                ..default()
            },
            ControlPointDraggable {
                state: ControlPointState::None,
            },
        ));
    }

    //curve
    commands.spawn(
        Curve(CubicBezier::new([positions]).to_curve())
    );
}

fn update_states(
    cameras: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut raycast: Raycast,
    mut control_points: Query<(&Transform, &mut ControlPointDraggable)>,
    mut ctrl_pts_plane: Query<&mut Transform, (With<ControlPointsPlane>, Without<Cursor>, Without<ControlPointDraggable>)>
) {
    let (camera, camera_transform) = cameras.single();
    let Some(cursor_position) = windows.single().cursor_position() else {return; };
    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {return;};
    
    let Ok(mut ctrl_pts_plane_trm) = ctrl_pts_plane.get_single_mut() else {return;};

    let intersections = raycast.cast_ray(
        ray,
        &RaycastSettings {
            filter: &|e| control_points.contains(e),
            ..default()
        },
    );
    
    if intersections.len() > 0 {
        if let Ok((ctrl_pt_trm, mut ctrl_pt_draggable)) 
        = control_points.get_mut(intersections[0].0) {
            ctrl_pt_draggable.state = if buttons.pressed(MouseButton::Left) {
                if ctrl_pt_draggable.state == ControlPointState::None {
                    ctrl_pts_plane_trm.translation = ctrl_pt_trm.translation;
                }
                ControlPointState::Drag
            } else {
                ControlPointState::None
            };
        }
    }
}

//if dragging cp
//raycast, detect cp
//clip plane to the cp
//raycast, detect plane
//move cp on plane surface

fn update_positions(
    cameras: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut cursors: Query<&mut Transform, (With<Cursor>, Without<ControlPointDraggable>)>,
    planes: Query<(&ControlPointsPlane, &Transform), Without<Cursor>>,
    mut ctrl_pts_transforms: Query<
        (&mut Transform, &ControlPointDraggable), 
        Without<ControlPointsPlane>,
    >,
    mut raycast: Raycast,
) {
    let Ok(mut cursor) = cursors.get_single_mut() else {return;};

    for (mut ctrl_pt_trm, ctrl_pt_cmp) in ctrl_pts_transforms.iter_mut() {
        if let ControlPointState::Drag = ctrl_pt_cmp.state {
            //make cursor move on plane if dragging

            let (camera, camera_transform) = cameras.single();
            let Some(cursor_position) = windows.single().cursor_position() else {return; };
            let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {return;};

            let Ok((plane_cmp, plane_trm)) = planes.get_single() else {return;};

            let intersections = raycast.cast_ray(
                ray,
                &RaycastSettings {
                    filter: &|e| planes.contains(e),
                    ..default()
                },
            );

            if intersections.len() > 0 {
                //changing cursor position again? this is stupid. should check if ctrl pt draggable maybe?
                cursor.translation = intersections[0].1.position();
                ctrl_pt_trm.translation = cursor.translation
            }

        }
    }
}

fn draw_spline(
    control_points_transforms: Query<&Transform, With<ControlPointDraggable>>,
    mut gizmos: Gizmos,
) {
    let pts: Vec<Vec3> = control_points_transforms
        .iter()
        .map(|t| t.translation)
        .collect();

    if let Ok(array) = pts[..].try_into() {
        let curve = CubicBezier::new(vec![array]).to_curve();
        let curve_pts: Vec<_> = curve.iter_positions(100).collect();

        gizmos.linestrip(curve_pts, Color::WHITE);
    }
}
