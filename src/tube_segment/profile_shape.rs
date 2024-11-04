use std::vec;
use bevy::math::*;

pub struct Vertex {
	pub point: Vec2,
	pub normal: Vec2,
	pub u: f32,	//when we will extrurde the shape we will get uv's from that
	//vertex color...
}

//a shape that is translated across curve after which every step profile would be created
pub struct ProfileShape {
	pub vertices: Vec<Vertex>,
	pub line_indices: Vec<usize>,
}

impl ProfileShape {
	pub fn vertex_count(&self) -> usize {
		self.vertices.len()
	}

	pub fn line_count(&self) -> usize {
		self.line_indices.len()
	}
	
	//length of this shape. all lines lengths summed up in u's
	pub fn calc_u_span(&self) -> f32 {
		let mut dist: f32 = 0.;
		let line_count = self.line_count();
		for i in (0..line_count).step_by(2) {
			let u_a = self.vertices[self.line_indices[i]].point;
			let u_b = self.vertices[self.line_indices[i + 1]].point;
			dist += (u_a - u_b).length();
		}
		
		dist
	}

	pub fn circle_8 () -> Self {
		let sqrt = 1./f32::sqrt(2.);
		// let sin45_half = f32::sin(f32::to_radians(45.)/2.);
		// let cos45_half = f32::cos(f32::to_radians(45.)/2.);
		Self {
			vertices: vec![
				//TODO: set correct normals
				Vertex{ point: Vec2::new(0., 1.),       normal:  Vec2::Y, u: 1.000},
				Vertex{ point: Vec2::new(0., 1.),       normal:  Vec2::Y, u: 0.000},
				Vertex{ point: Vec2::new(-sqrt, sqrt),  normal: -Vec2::X, u: 0.125},
				Vertex{ point: Vec2::new(-sqrt, sqrt),  normal: -Vec2::X, u: 0.125},
				Vertex{ point: Vec2::new(-1., 0.),      normal: -Vec2::X, u: 0.125 * 2.},
				Vertex{ point: Vec2::new(-1., 0.),      normal: -Vec2::X, u: 0.125 * 2.},
				Vertex{ point: Vec2::new(-sqrt, -sqrt), normal: -Vec2::Y, u: 0.125 * 3.},
				Vertex{ point: Vec2::new(-sqrt, -sqrt), normal: -Vec2::Y, u: 0.125 * 3.},
				Vertex{ point: Vec2::new(0., -1.),      normal: -Vec2::Y, u: 0.125 * 4.},
				Vertex{ point: Vec2::new(0., -1.),      normal: -Vec2::Y, u: 0.125 * 4.},
				Vertex{ point: Vec2::new(sqrt, -sqrt),  normal:  Vec2::X, u: 0.125 * 5.},
				Vertex{ point: Vec2::new(sqrt, -sqrt),  normal:  Vec2::X, u: 0.125 * 5.},
				Vertex{ point: Vec2::new(1., 0.),       normal:  Vec2::X, u: 0.125 * 6.},
				Vertex{ point: Vec2::new(1., 0.),       normal:  Vec2::X, u: 0.125 * 6.},
				Vertex{ point: Vec2::new(sqrt, sqrt),   normal:  Vec2::Y, u: 0.125 * 7.},
				Vertex{ point: Vec2::new(sqrt, sqrt),   normal:  Vec2::Y, u: 0.125 * 7.},
			],
			line_indices: vec![
				1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0
			]
		}
	}
}