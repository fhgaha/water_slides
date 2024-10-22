use bevy::prelude::*;
use bevy_mod_raycast::prelude::*;

use crate::game::{ControlPointsPlane, Cursor};

pub struct RoadSegmentPlugin;

impl Plugin for RoadSegmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(
            Update,
            (
                (update_states, update_positions).chain(), 
                // draw_spline
                 draw_spline_using_road_segment
            ),
        );
    }
}

#[derive(Component)]
struct RoadSegment {
    curve: CubicCurve<Vec3>,
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

    commands
        .spawn(
            (
                TransformBundle::default(),
                RoadSegment {
                    curve: CubicBezier::new([positions]).to_curve(),
                }
            )
        ).with_children(|parent| {
            for p in positions {
                parent.spawn((
                    PbrBundle {
                        mesh: meshes.add(Sphere::new(1.)),
                        material: materials.add(Color::srgb(1., 1., 1.)),
                        // transform: Transform::from_xyz(p.x, p.y, p.z),
                        transform: Transform::from_translation(p),
                        ..default()
                    },
                    ControlPointDraggable {
                        state: ControlPointState::None,
                    },
                ));
            }
        });
}

fn update_states(
    cameras: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut raycast: Raycast,
    mut control_points: Query<(&Transform, &mut ControlPointDraggable)>,
    mut ctrl_pts_plane: Query<
        &mut Transform, 
        (With<ControlPointsPlane>, Without<Cursor>, Without<ControlPointDraggable>)
    >
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
    mut raycast: Raycast,
    mut cursors: Query<&mut Transform, (With<Cursor>, Without<ControlPointDraggable>)>,
    planes: Query<(&ControlPointsPlane, &Transform), Without<Cursor>>,
    mut ctrl_pts_transforms: Query<
        (&mut Transform, &ControlPointDraggable), 
        Without<ControlPointsPlane>,
    >,
) {
    for (mut ctrl_pt_trm, ctrl_pt_cmp) in ctrl_pts_transforms.iter_mut() {
        if let ControlPointState::Drag = ctrl_pt_cmp.state {
            //make cursor move on plane if dragging

            let (camera, camera_transform) = cameras.single();
            let Some(cursor_position) = windows.single().cursor_position() else {return; };
            let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {return;};
            
            let Ok(mut cursor) = cursors.get_single_mut() else {return;};
            
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

fn draw_spline_using_road_segment(
    mut road_segment_query: Query<(Entity, &Children), With<RoadSegment>>,
    mut control_point_query: Query<&mut Transform, With<ControlPointDraggable>>,
    mut gizmos: Gizmos,
) {
    for (road_segm_entity, children) in &mut road_segment_query {
        let mut positions: [[Vec3; 4]; 1] = [[Vec3::INFINITY, Vec3::INFINITY, Vec3::INFINITY, Vec3::INFINITY]]; 

        for (i, child) in children.iter().enumerate(){
            if let Ok(mut transform) = control_point_query.get_mut(*child) {
                positions[0][i] = transform.translation;
            }
        }

        if positions[0].iter().all(|&pos| pos == Vec3::INFINITY) {return};

        gizmos.linestrip(
            CubicBezier::new(positions)
                .to_curve()
                .iter_positions(100)
                .collect::<Vec<Vec3>>(), 
            Color::WHITE
        );
    }
}
