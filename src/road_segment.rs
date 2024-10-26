use bevy::prelude::*;
use bevy_mod_raycast::prelude::*;
use my_ui::*;

use crate::{game::{ControlPointsPlane, Cursor}, my_ui};

pub struct RoadSegmentPlugin;

impl Plugin for RoadSegmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(
            Update,
            (
                // update_road_segment_pts,
                (update_states, update_positions).chain(), 
                // draw_spline
                draw_curve_using_road_segment,
                // sphere_along_curve_move_with_time,
                sphere_along_curve_move_from_ui_input
            )
        );
    }
}

#[derive(Component)]
struct RoadSegment {
    curve: CubicBezier<Vec3>,
    pts: [Entity; 4]
}

impl Default for RoadSegment {
    fn default() -> Self {
        Self{
            curve: CubicBezier::new([[Vec3::INFINITY, Vec3::INFINITY, Vec3::INFINITY, Vec3::INFINITY]]),
            pts: [Entity::from_bits(0); 4]
        }
    }
}

impl RoadSegment {
    fn curve_pts(&mut self, transforms: &Query<&Transform>, subdivisions: usize) -> Vec<Vec3> {
        let positions: [Vec3; 4] = self.pts
            .iter()
            .map(|pt| transforms.get(*pt).unwrap().translation)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        self.curve = CubicBezier::new([positions]);

        self.curve
            .to_curve()
            .iter_positions(subdivisions)
            .collect::<Vec<Vec3>>()
    }
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

#[derive(Component)]
struct MovingSphere;

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

    //control points
    let control_pts_ids: [Entity; 4] = positions.map(|p|{
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Sphere::new(1.)),
                material: materials.add(Color::srgb(1., 1., 1.)),
                transform: Transform::from_translation(p),
                ..default()
            },
            ControlPointDraggable {
                state: ControlPointState::None,
            },
        ))
        .id()
    });

    //road segment
    commands
        .spawn((
                SpatialBundle::default(),
                RoadSegment {
                    curve: CubicBezier::new([positions]),
                    pts: control_pts_ids
                }
        ))
        .push_children(&control_pts_ids);

    //moving sphere
    commands.spawn((PbrBundle{
        mesh: meshes.add(Sphere::new(1.0)),
        material: materials.add(Color::srgba(1., 0., 0., 0.5)),
        transform: Transform::from_translation(positions[0]),
        ..default()
    },
    MovingSphere));
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

#[allow(dead_code)]
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

fn draw_curve_using_road_segment(
    mut road_segments: Query<&mut RoadSegment>,
    transforms: Query<&Transform>,    
    mut gizmos: Gizmos,
) {
    for mut rs in road_segments.iter_mut() {
        gizmos.linestrip(
            rs.curve_pts(&transforms, 100), 
            Color::WHITE
        );
    }
}

#[allow(dead_code)]
fn sphere_along_curve_move_with_time(
    time: Res<Time>, 
    road_segments: Query<&RoadSegment>,
    mut moving_spheres: Query<&mut Transform, With<MovingSphere>>,
){
    let t = (time.elapsed_seconds().sin() + 1.) / 2.;

    for rs in road_segments.iter() {
        let pos = rs.curve.to_curve().position(t);
        for mut s in moving_spheres.iter_mut() {
            s.translation = pos;
        }
    }
}

fn sphere_along_curve_move_from_ui_input(
    ui_state: Res<UiState>,
    road_segments: Query<&RoadSegment>,
    mut moving_spheres: Query<&mut Transform, With<MovingSphere>>,
){
    for rs in road_segments.iter() {
        let pos = rs.curve.to_curve().position(ui_state.value);
        for mut s in moving_spheres.iter_mut() {
            s.translation = pos;
        }
    }
}