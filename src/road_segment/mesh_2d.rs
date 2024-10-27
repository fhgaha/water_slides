use std::vec;
use bevy::math::*;
use std::num;

struct Vertex {
	pub point: Vec2,
	pub normal: Vec2,
	pub u: f32,	//when we will extrurde the shape we will get uv's from that
	//vertex color...
}

impl Vertex {
	pub fn new(point: Vec2, normal: Vec2, u:f32) -> Self {
		Self {
			point,
			normal,
			u
		}
	}
}

//a shape that is translated across curve after which every step profile would be created
pub struct Mesh2d {
	pub vertices: Vec<Vertex>,
	pub line_indices: Vec<i32>,
}

impl Mesh2d {
	pub fn circle_8 () -> Self {
		let sqrt = -1./f32::sqrt(2.);
		let sin45_half = f32::sin(f32::to_radians(45.)/2);
		let cos45_half = f32::cos(f32::to_radians(45.)/2);
		Self {
			vertices: vec![
				//TODO: set correct normals
				Vertex::new(Vec2::new(0., 1.), 		  Vec2::Y, 0.),
				Vertex::new(Vec2::new(0., 1.),        Vec2::Y, 0.),
				Vertex::new(Vec2::new(-sqrt, sqrt),  -Vec2::X, 0.),
				Vertex::new(Vec2::new(-sqrt, sqrt),  -Vec2::X, 0.),
				Vertex::new(Vec2::new(-1., 0.),      -Vec2::X, 0.),
				Vertex::new(Vec2::new(-1., 0.),      -Vec2::X, 0.),
				Vertex::new(Vec2::new(-sqrt, -sqrt), -Vec2::Y, 0.),
				Vertex::new(Vec2::new(-sqrt, -sqrt), -Vec2::Y, 0.),
				Vertex::new(Vec2::new(0., -1.),      -Vec2::Y, 0.),
				Vertex::new(Vec2::new(0., -1.),      -Vec2::Y, 0.),
				Vertex::new(Vec2::new(sqrt, -sqrt),   Vec2::X, 0.),
				Vertex::new(Vec2::new(sqrt, -sqrt),   Vec2::X, 0.),
				Vertex::new(Vec2::new(1., 0.),        Vec2::X, 0.),
				Vertex::new(Vec2::new(1., 0.),        Vec2::X, 0.),
				Vertex::new(Vec2::new(sqrt, sqrt),    Vec2::Y, 0.),
				Vertex::new(Vec2::new(sqrt, sqrt),    Vec2::Y, 0.),
			],
			line_indices: vec![
				//16, ??
				15, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14
			]
		}
	}
}