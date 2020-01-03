use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

pub type Float = f64;
#[derive(Copy, Clone, Debug)]
pub struct Vector3 {
    pub items: [Float; 3],
}

impl Vector3 {
    // Constructors
    pub fn zero() -> Self {
        Vector3 { items: [0.0; 3] }
    }
    pub fn unit_x() -> Self {
        Vector3 {
            items: [1.0, 0.0, 0.0],
        }
    }
    pub fn unit_y() -> Self {
        Vector3 {
            items: [0.0, 1.0, 0.0],
        }
    }
    pub fn unit_z() -> Self {
        Vector3 {
            items: [0.0, 0.0, 1.0],
        }
    }
    pub fn from_vec(items: [Float; 3]) -> Self {
        Vector3 { items }
    }
    pub fn from_polar(latitude: Float, longitude: Float, magnitude: Float) -> Self {
        // Principal values of input:
        // -PI/2 <= latitude <= PI/2
        // -PI <= longitude <= PI
        Vector3::unit_x()
            .rotated_around_y(latitude)
            .rotated_around_z(longitude)
            * magnitude
    }

    // Operations
    pub fn dot(&self, other: &Self) -> Float {
        self.items
            .iter()
            .zip(other.items.iter())
            .fold(0.0, |acc, (a, b)| acc + (*a) * (*b))
    }
    pub fn norm_squared(&self) -> Float {
        self.dot(self)
    }
    pub fn norm(&self) -> Float {
        self.norm_squared().sqrt()
    }
    // Rotations
    pub fn rotated_around_x(&self, angle: Float) -> Vector3 {
        Vector3 {
            items: [
                self.items[0],
                self.items[1] * Float::cos(angle) - self.items[2] * Float::sin(angle),
                self.items[1] * Float::sin(angle) + self.items[2] * Float::cos(angle),
            ],
        }
    }
    pub fn rotated_around_y(&self, angle: Float) -> Vector3 {
        Vector3 {
            items: [
                self.items[0] * Float::cos(angle) - self.items[2] * Float::sin(angle),
                self.items[1],
                self.items[0] * Float::sin(angle) + self.items[2] * Float::cos(angle),
            ],
        }
    }
    pub fn rotated_around_z(&self, angle: Float) -> Vector3 {
        Vector3 {
            items: [
                self.items[0] * Float::cos(angle) - self.items[1] * Float::sin(angle),
                self.items[0] * Float::sin(angle) + self.items[1] * Float::cos(angle),
                self.items[2],
            ],
        }
    }
    pub fn rotated_around_xyz(&self, angle_x: Float, angle_y: Float, angle_z: Float) -> Vector3 {
        self.rotated_around_x(angle_x)
            .rotated_around_y(angle_y)
            .rotated_around_z(angle_z)
    }
}

impl std::fmt::Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}, {}]", self.items[0], self.items[1], self.items[2])
    }
}

// region Vector3<N> {AddAssign<Self>, SubAssign<Self>, MulAssign<Float>, DivAssign<Float>} impls
impl AddAssign for Vector3 {
    fn add_assign(&mut self, other: Self) {
        for (selfref, val) in self.items.iter_mut().zip(&other.items) {
            *selfref += *val;
        }
    }
}
impl SubAssign for Vector3 {
    fn sub_assign(&mut self, other: Self) {
        for (selfref, val) in self.items.iter_mut().zip(&other.items) {
            *selfref -= *val;
        }
    }
}

impl MulAssign<Float> for Vector3 {
    fn mul_assign(&mut self, scalar: Float) {
        for item in self.items.iter_mut() {
            *item *= scalar;
        }
    }
}
impl DivAssign<Float> for Vector3 {
    fn div_assign(&mut self, scalar: Float) {
        for item in self.items.iter_mut() {
            *item /= scalar;
        }
    }
}
// endregion

// region Vector3 {Add<Self>, Sub<Self>, Mul<Float>, Div<Float>} impls
impl Add for Vector3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut result = self;
        result += other;
        result
    }
}
impl Sub for Vector3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let mut result = self;
        result -= other;
        result
    }
}
impl Mul<Float> for Vector3 {
    type Output = Self;
    fn mul(self, scalar: Float) -> Self {
        let mut result = self;
        result *= scalar;
        result
    }
}
impl Div<Float> for Vector3 {
    type Output = Self;
    fn div(self, scalar: Float) -> Self {
        let mut result = self;
        result /= scalar;
        result
    }
}
// endregion

// region Float {Mul<Vector3>, Div<Vector3>} impls
impl Mul<Vector3> for Float {
    type Output = Vector3;
    fn mul(self, vector: Vector3) -> Vector3 {
        vector * self
    }
}
impl Div<Vector3> for Float {
    type Output = Vector3;
    fn div(self, vector: Vector3) -> Vector3 {
        vector / self
    }
}
// endregion
