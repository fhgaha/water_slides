use bevy::math::{Quat, Vec3};

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

    pub fn local_to_world(self, local_space_pos: Vec3) -> Vec3 {
        let world_pos = self.rot * local_space_pos;
        self.pos + world_pos
    }
}
 