use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::process::Output;

pub trait VectorType: Copy + Clone + Default {}

impl VectorType for f32 {}

impl VectorType for f64 {}

#[derive(Debug, Default, Copy, Clone)]
pub struct Vec3<T>
where
    T: VectorType,
{
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T>
where
    T: VectorType,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T> Mul for Vec3<T>
where
    T: VectorType + Mul<Output = T>,
{
    type Output = Vec3<T::Output>;
    fn mul(self, other: Self) -> Self::Output {
        Vec3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl Mul<f32> for Vec3<f32> {
    type Output = Self;
    fn mul(self, other: f32) -> Self::Output {
        Vec3::new(self.x * other, self.y * other, self.z * other)
    }
}

impl Mul<f64> for Vec3<f64> {
    type Output = Self;
    fn mul(self, other: f64) -> Self::Output {
        Vec3::new(self.x * other, self.y * other, self.z * other)
    }
}

impl<T> MulAssign for Vec3<T>
where
    T: VectorType + Mul<Output = T>,
{
    fn mul_assign(&mut self, other: Self) {
        self.x = self.x * other.x;
        self.y = self.y * other.y;
        self.z = self.z * other.z;
    }
}

impl<T> Add for Vec3<T>
where
    T: VectorType + Add<Output = T>,
{
    type Output = Vec3<T::Output>;
    fn add(self, other: Self) -> Self::Output {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl<T> AddAssign for Vec3<T>
where
    T: VectorType + Add<Output = T>,
{
    fn add_assign(&mut self, other: Self) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
        self.z = self.z + other.z;
    }
}

impl<T> Div for Vec3<T>
where
    T: VectorType + Div<Output = T>,
{
    type Output = Vec3<T::Output>;
    fn div(self, other: Self) -> Self::Output {
        Vec3::new(self.x / other.x, self.y / other.y, self.z / other.z)
    }
}

impl<T> DivAssign for Vec3<T>
where
    T: VectorType + Div<Output = T>,
{
    fn div_assign(&mut self, other: Self) {
        self.x = self.x / other.x;
        self.y = self.y / other.y;
        self.z = self.z / other.z;
    }
}

impl<T> Sub for Vec3<T>
where
    T: VectorType + Sub<Output = T>,
{
    type Output = Vec3<T::Output>;
    fn sub(self, other: Self) -> Self::Output {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl<T> Neg for Vec3<T>
where
    T: VectorType + Neg<Output = T>,
{
    type Output = Vec3<T::Output>;
    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl<T> SubAssign for Vec3<T>
where
    T: VectorType + Sub<Output = T>,
{
    fn sub_assign(&mut self, other: Self) {
        self.x = self.x - other.x;
        self.y = self.y - other.y;
        self.z = self.z - other.z;
    }
}

impl<T> Vec3<T>
where
    T: VectorType
        + Div<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Add<Output = T>
        + Neg<Output = T>,
{
    pub fn dot(&self, other: Vec3<T>) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Vec3<T>) -> Vec3<T> {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            -(self.x * other.z - self.z * other.x),
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn squared_length(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn r(&self) -> T {
        self.x
    }

    pub fn g(&self) -> T {
        self.y
    }

    pub fn b(&self) -> T {
        self.z
    }
}

impl Vec3<f32> {
    pub fn length(&self) -> f32 {
        f32::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }
}

impl Vec3<f64> {
    pub fn length(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }
}
