use vector::Vec3;

pub struct Ray {
    A : Vec3,
    B : Vec3
}

impl Ray {
    pub fn origin(&self) -> Vec3 {
        A
    }

    pub fn direction(&self) -> Vec3 {
        B
    }

    pub fn point_at_parameter(&self, t : f32) -> Vec3 {
        self.A + t*self.B
    }
}