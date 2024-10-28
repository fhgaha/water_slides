use bevy::{math::{Quat, Vec3}, prelude::Vec2};

#[derive(Clone, Copy)]
pub struct OrientedPoint {
    pub pos: Vec3,
    pub rot: Quat
}

impl OrientedPoint {
    pub fn from_forward(pos: Vec3, forward: Vec3) -> Self {
        Self {
            pos,
            rot: Quat::from_rotation_arc(Vec3::Z, forward)
        }
    }

    pub fn local_to_world_pos(self, local_space_pos: Vec2) -> Vec3 {
        let world_pos = self.rot * local_space_pos.extend(0.);
        self.pos + world_pos
    }

    pub fn local_to_world_vect(self, local_space_pos: Vec2) -> Vec3 {
        self.rot * local_space_pos.extend(0.)
    }
}
   