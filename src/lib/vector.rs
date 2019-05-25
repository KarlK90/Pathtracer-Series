use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Default, Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x: x, y: y, z: z }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Self) -> Self::Output {
        Vec3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: f32) -> Self::Output {
        Vec3::new(self.x * other, self.y * other, self.z * other)
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        self.x = self.x * other.x;
        self.y = self.y * other.y;
        self.z = self.z * other.z;
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Self) -> Self::Output {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
        self.z = self.z + other.z;
    }
}

impl Div for Vec3 {
    type Output = Vec3;
    fn div(self, other: Self) -> Self::Output {
        Vec3::new(self.x / other.x, self.y / other.y, self.z / other.z)
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Self) {
        self.x = self.x / other.x;
        self.y = self.y / other.y;
        self.z = self.z / other.z;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Self) -> Self::Output {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        self.x = self.x - other.x;
        self.y = self.y - other.y;
        self.z = self.z - other.z;
    }
}

impl Vec3 {
    pub fn dot(&self, other: Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3::new(
            (self.y * other.z - self.z * other.y),
            -(self.x * other.z - self.z * other.x),
            (self.x * other.y - self.y * other.x),
        )
    }

    pub fn length(&self) -> f32 {
        f32::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub fn squared_length(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn r(&self) -> f32 {
        self.x
    }

    pub fn g(&self) -> f32 {
        self.y
    }

    pub fn b(&self) -> f32 {
        self.z
    }
}
