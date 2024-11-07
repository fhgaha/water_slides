mod oriented_point;
mod profile_shape;

use core::str;
use std::ops::DerefMut;
use bevy::{
    color::palettes::css::{AQUA, YELLOW}, 
    prelude::*, 
    render::{
        mesh::{
            Indices, PrimitiveTopology
        }, 
        render_asset::RenderAssetUsages
    }
};
use bevy_mod_raycast::prelude::*;
use my_ui::*;
use oriented_point::OrientedPoint;
use profile_shape::*;
use crate::{game::{my_cursor::MyCursor, ControlPointsPlane}, my_ui};

pub struct TubeSegmentPlugin;

impl Plugin for TubeSegmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(
            Update,
                // update_road_segment_pts,
                (
                    update_states, 
                    update_positions, 
                    // draw_spline,
                    draw_curve_using_road_segment,
                    draw_profile,
                    generate_mesh,
                ).chain()
        );
    }
}

#[derive(Component)]
struct RoadSegment {
    curve: CubicBezier<Vec3>,
    pts_ids: [Entity; 4],
    start_pt_id: Option<Entity>,
    end_pt_id: Option<Entity>,
}

impl Default for RoadSegment {
    fn default() -> Self {
        Self{
            curve: CubicBezier::new([[Vec3::INFINITY, Vec3::INFINITY, Vec3::INFINITY, Vec3::INFINITY]]),
            pts_ids: [Entity::from_bits(0); 4],
            start_pt_id: None,
            end_pt_id: None 
        }
    }
}

impl RoadSegment {
    fn transforms_to_positions(&self, transforms: &Query<&Transform>) -> Vec<Vec3> {
        let points: Vec<Vec3> = self.pts_ids
            .iter()
            .map(|pt_id| transforms.get(*pt_id).unwrap().translation)
            .collect();

        points
    }

    fn calc_and_store_curve_return_curve_pts(&mut self, positions: &Vec<Vec3>, subdivisions: usize) -> Vec<Vec3> {
        let positions_as_arr: [Vec3; 4] = positions
            .as_slice()
            .try_into()
            .unwrap();

        self.curve = CubicBezier::new([positions_as_arr]);

        self.curve
            .to_curve()
            .iter_positions(subdivisions)
            .collect::<Vec<Vec3>>()
    }

    fn get_bezier_oriented_point(&self, t: f32) -> OrientedPoint {
        OrientedPoint::from_forward(
            self.curve.to_curve().position(t), 
            self.curve.to_curve().velocity(t).normalize()
        )
    }

    #[allow(dead_code)]
    fn get_profile_center_and_lines(&self, t: f32, profile_shape: &ProfileShape) -> (OrientedPoint, Vec<(Vec3, Vec3)>) {
        let op = self.get_bezier_oriented_point(t);
        let shape_2d = profile_shape;
        
        //lines
        let verts: Vec<Vec3> = shape_2d.vertices.iter()
            .map(|v| op.local_to_world_pos(v.point))
            .collect();
        
        let line_pairs: Vec<(Vec3, Vec3)> = shape_2d.line_indices
            .chunks(2)
            .map(|line_idx_pair| (verts[line_idx_pair[0]], verts[line_idx_pair[1]]))
            .collect();
        
        (op, line_pairs)
    }
    
    #[allow(dead_code)]
    fn get_approx_len(&self) -> f32 {
        const PRESCISION: usize = 8;
        let mut points: Vec<Vec3> = vec![];
        
        for i in 0..PRESCISION {
            let t = i as f32 / (PRESCISION - 1) as f32;
            points.push(self.get_bezier_oriented_point(t).pos);
        }
        
        let mut dist: f32 = 0.;
        for i in 0..(PRESCISION - 1) {
            let a = points[i];
            let b = points[i + 1];
            dist += Vec3::distance(a, b);
        }
        
        dist
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
struct MovingSphere;

#[derive(Component)]
struct CustomMesh;


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let positions = [
        Vec3::new(-10., 0.,  10.),
        Vec3::new(-10., 0., -10.),
        Vec3::new( 10., 0., -10.),
        Vec3::new( 10., 0.,  10.),
    ];

    //control points
    // let control_pts_ids: [Entity; 4] = positions.map(|p|{
    //     commands.spawn((
    //         PbrBundle {
    //             mesh: meshes.add(Sphere::new(1.)),
    //             material: materials.add(Color::srgba(1., 1., 1., 0.2)),
    //             transform: Transform::from_translation(p),
    //             ..default()
    //         },
    //         ControlPointDraggable {
    //             state: ControlPointState::None,
    //         },
    //     ))
    //     .id()
    // });

    let mut control_pts_ids: [Entity; 4] = [Entity::PLACEHOLDER; 4];

    for i in 0..positions.len() {
        control_pts_ids[i] = commands.spawn((
            Name::new(format!("Control Point {i}")),
            PbrBundle {
                mesh: meshes.add(Sphere::new(1.)),
                material: materials.add(Color::srgba(1., 1., 1., 0.2)),
                transform: Transform::from_translation(positions[i]),
                ..default()
            },
            ControlPointDraggable {
                state: ControlPointState::None,
            },
        ))
        .id()
    }

    //moving sphere
    commands.spawn((
        PbrBundle{
            mesh: meshes.add(Sphere::new(0.2)),
            material: materials.add(Color::srgba(1., 0., 0., 1.)),
            transform: Transform::from_translation(positions[0]),
            ..default()
        },
        MovingSphere
    ));


    //generated mesh

    let texture_handle: Handle<Image> = asset_server.load("textures/uv_mapper.png");
    let mesh_handle: Handle<Mesh> = meshes.add(
        Mesh::new(
            PrimitiveTopology::TriangleList, 
            RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vec![
            Vec3::new(-1.,  1., 0.),
            Vec3::new( 1.,  1., 0.),
            Vec3::new( 1., -1., 0.),
            Vec3::new(-1., -1., 0.),
        ])
        .with_inserted_indices(Indices::U32(vec![0, 3, 1, 1, 3, 2]))
        .with_computed_normals()
    );

    // Render the mesh with the custom texture using a PbrBundle, add the marker.
    commands.spawn((
        PbrBundle {
            mesh: mesh_handle.clone(),
            material: materials.add(StandardMaterial {
                // base_color_texture: Some(texture_handle),
                base_color: Color::Srgba(AQUA), 
                ..default()
            }),
            ..default()
        },
        CustomMesh,
    ));

    //road segment
    commands
        .spawn((
                SpatialBundle::default(),
                RoadSegment {
                    curve: CubicBezier::new([positions]),
                    pts_ids: control_pts_ids,
                    start_pt_id: Some(control_pts_ids[0]),
                    end_pt_id: Some(control_pts_ids[3])
                }
        ))
        .push_children(&control_pts_ids);
}

fn update_states(
    cameras: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut raycast: Raycast,
    mut control_points: Query<(&Transform, &mut ControlPointDraggable)>,
    mut ctrl_pts_plane: Query<
        &mut Transform, 
        (With<ControlPointsPlane>, Without<MyCursor>, Without<ControlPointDraggable>)
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
    mut cursors: Query<&mut Transform, (With<MyCursor>, Without<ControlPointDraggable>)>,
    planes: Query<(&ControlPointsPlane, &Transform), Without<MyCursor>>,
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
        let positions = rs.transforms_to_positions(&transforms);

        gizmos.linestrip(
            rs.calc_and_store_curve_return_curve_pts(&positions, 100), 
            Color::WHITE
        );
    }
}

#[allow(dead_code)]
fn draw_curve_using_road_segment_other_curve_options(
    mut road_segments: Query<&mut RoadSegment>,
    transforms: Query<&Transform>,    
    mut gizmos: Gizmos,
    ui_state: Res<UiState>,
) {
    for rs in road_segments.iter_mut() {
        
         let positions: [Vec3; 4] = rs.pts_ids
            .iter()
            .map(|pt| transforms.get(*pt).unwrap().translation)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let curve = CubicBezier::new([positions]);
        // let curve = CubicBSpline::new(positions);
        // let curve = CubicCardinalSpline::new(0.7, positions); //this looks good
        // let curve = CubicHermite::new(positions, [Vec3::splat(10.); 4]);

        // let weights = [10.0, 10.0, 20.0, 10.0];
        // let knots = [0.0, 0.0, 10.0, 20.0, 30.0, 40.0, 50.0, 50.0];
        // let curve = CubicNurbs::new(positions, Some(weights), Some(knots))
        //     .expect("NURBS construction failed!");

        let subdivisions: usize = ui_state.sections_amnt.try_into().unwrap();

        let positions = curve
            .to_curve()
            .iter_positions(subdivisions)
            .collect::<Vec<Vec3>>();

        // let positions = rs.curve_pts(&transforms, 100);
        
        gizmos.linestrip(
            positions, 
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

#[allow(dead_code)]
fn draw_shape(gizmos: &mut Gizmos<'_, '_>, op: OrientedPoint, local_space_pos: Vec2) {
    const RED: Srgba = bevy::color::palettes::basic::RED;
    gizmos.sphere(op.local_to_world_pos(local_space_pos), op.rot, 0.2, RED).resolution(8);
}

fn draw_profile(
    ui_state: Res<UiState>,
    road_segments: Query<&RoadSegment>,
    mut moving_spheres: Query<&mut Transform, With<MovingSphere>>,
    mut gizmos: Gizmos
){
    for rs in road_segments.iter() {
        for mut sphere in moving_spheres.iter_mut() {
            
            let t = ui_state.t_value;
            let shape2d = ProfileShape::circle_8();
            
            let (center, profile_edges) 
                = rs.get_profile_center_and_lines(t, &shape2d);

            for (from, to) in profile_edges {
                gizmos.line(from, to, Color::Srgba(YELLOW));
            }

            sphere.translation = center.pos;
            //lock Y for this quat when you dont want the thing to rotate around movement direction
            sphere.rotation = center.rot;
            
            gizmos.axes(*sphere.deref_mut(), 4.);
         }
    
    // https://github.com/Kurble/bevy_mod_inverse_kinematics

    //cubic
    //quadratic
    //quentic
    // Catmull-Rom
    //natural cubic spline
    }
}

fn generate_mesh(
    mut road_segments: Query<&mut RoadSegment>,
    asset_server: Res<AssetServer>,
    mut mesh_asset_server: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut control_pts: Query<&mut Transform, With<ControlPointDraggable>>,
    mut query: Query<(&mut CustomMesh, &mut Handle<Mesh>, &mut Handle<StandardMaterial>)>,
    ui_state: Res<UiState>,
    
    mut config_store: ResMut<GizmoConfigStore>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    // if control_pts.iter_mut().all(|p| !p.is_changed()) {return;};
    
    //toggle gizmos drawing over objects
    if keyboard.just_pressed(KeyCode::KeyD) {
        for (_, config, _) in config_store.iter_mut() {
            config.depth_bias = if config.depth_bias == 0. { -1. } else { 0. };
        }
    }

    for mut rs in road_segments.iter_mut() {
        for (mut _custom_mesh, mut mesh_handle, mut material_handle) in query.iter_mut(){
            
            let shape2d = ProfileShape::circle_8();
            
            let mut control_pts_positions: Vec<Vec3> = rs.pts_ids
                .iter()
                .map(|pt_id| control_pts.get(*pt_id).unwrap().translation)
                .collect();
            
            let control_pts_trms: Vec<&Transform> = control_pts
                .iter()
                .map(|x| x)
                .collect();
            
            //locks mid points to start and end
            //need to change its transform z scale to see the effect
            control_pts_positions[1] = control_pts_trms[0].transform_point(-Vec3::Z * control_pts_trms[0].scale.z);
            control_pts_positions[2] = control_pts_trms[3].transform_point( Vec3::Z * control_pts_trms[3].scale.z);
            
            let sections_amnt = ui_state.sections_amnt.try_into().unwrap();

            rs.calc_and_store_curve_return_curve_pts(&control_pts_positions, sections_amnt);
    
            // Vertices
            //needed for uniforming uvs. not used here
            // let u_span = shape2d.calc_u_span();
            let edge_ring_count= sections_amnt;

            let mut verts = Vec::<Vec3>::new();
            //normals not used but let them be to show how are calculated
            let mut normals = Vec::<Vec3>::new();   
            let mut uvs = Vec::<Vec2>::new();
    
            for ring in 0..=edge_ring_count {
                
                let t: f32 = ring as f32 / (edge_ring_count - 1) as f32;
                let op = rs.get_bezier_oriented_point(t);
    
                for i in 0..shape2d.vertex_count() {
                    verts.push(op.local_to_world_pos(shape2d.vertices[i].point));
                    normals.push(op.local_to_world_vec(shape2d.vertices[i].normal));
                    //coefficient to uniform uvs. doesnt work, not used.
                    // let coeff = rs.get_approx_len()/u_span;
                    let coeff = 1.;
                    uvs.push(Vec2::new(shape2d.vertices[i].u , t * coeff));
                }
            }
            
            //
            //  A                   B
            //  .___________________.
            //  |    ring next      |
            //  |                /  |
            //  |             /     |
            //  |          /        |
            //  |       /           |
            //  |    /              |
            //  | /                 |
            //  .___________________.
            //  A    ring curr      B
            //
            // should be counter-clockwise in bevy
            //
    
            // Triangles
            let mut tri_indices = Vec::<u32>::new();
    
            for ring in 0..(edge_ring_count-1) {
                
                let root_idx = ring * shape2d.vertex_count();
                let root_idx_next = (ring + 1) * shape2d.vertex_count();
    
                for line in (0..shape2d.line_count()).step_by(2) {
                    
                    let line_idx_a = shape2d.line_indices[line];
                    let line_idx_b = shape2d.line_indices[line + 1];
                    
                    let curr_a = root_idx + line_idx_a;
                    let curr_b = root_idx + line_idx_b;
                    let next_a = root_idx_next + line_idx_a;
                    let next_b = root_idx_next + line_idx_b;
    
                    tri_indices.push(curr_a as u32);
                    tri_indices.push(curr_b as u32);
                    tri_indices.push(next_b as u32);
    
                    tri_indices.push(curr_a as u32);
                    tri_indices.push(next_b as u32);
                    tri_indices.push(next_a as u32);
                }
            }
            
            let new_mesh = Mesh::new(
                PrimitiveTopology::TriangleList, 
                RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
            )
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, verts)
            .with_inserted_indices(Indices::U32(tri_indices))
            // .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
            .with_computed_normals();
    
            *mesh_handle = mesh_asset_server.add(new_mesh);
        }
    }
}
