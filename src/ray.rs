use std::ops::Mul;

use crate::Vec3;
use crate::VectorType;

pub struct Ray<T>
where
    T: VectorType,
{
    a: Vec3<T>,
    b: Vec3<T>,
}

impl<T> Ray<T>
where
    T: VectorType,
{
    pub fn origin(&self) -> Vec3<T> {
        self.a
    }

    pub fn direction(&self) -> Vec3<T> {
        self.b
    }

    //pub fn point_at_parameter(&self, t : f32) -> Vec3 {
    //   self.A + t*self.B
    //}
}
