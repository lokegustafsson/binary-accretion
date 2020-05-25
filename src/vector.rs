use std::iter::FromIterator;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use std::slice::{Iter, IterMut};

pub type Float = f64;
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector3([Float; 3]);

impl Vector3 {
    // Constructors
    pub fn zero() -> Self {
        Vector3([0.0; 3])
    }
    pub fn unit_x() -> Self {
        Vector3([1.0, 0.0, 0.0])
    }
    pub fn unit_y() -> Self {
        Vector3([0.0, 1.0, 0.0])
    }
    pub fn unit_z() -> Self {
        Vector3([0.0, 0.0, 1.0])
    }
    pub fn from_polar(latitude: Float, longitude: Float, magnitude: Float) -> Self {
        // Principal values of input:
        // -PI/2 <= latitude <= PI/2
        // -PI <= longitude <= PI
        Vector3::unit_x()
            .rotated(Vector3::unit_y(), latitude)
            .rotated(Vector3::unit_x(), longitude)
            * magnitude
    }

    // Operation
    pub fn dot(self, other: Self) -> Float {
        self.iter().zip(other.iter()).map(|(a, b)| a * b).sum()
    }
    pub fn cross(self, other: Self) -> Vector3 {
        let [a1, a2, a3] = self.0;
        let [b1, b2, b3] = other.0;
        Vector3([a2 * b3 - a3 * b2, a3 * b1 - a1 * b3, a1 * b2 - a2 * b1])
    }
    pub fn norm_squared(self) -> Float {
        self.dot(self)
    }
    pub fn norm(self) -> Float {
        self.norm_squared().sqrt()
    }
    pub fn normalized(self) -> Vector3 {
        self / self.norm()
    }
    pub fn rotated(self, axis: Vector3, angle: Float) -> Vector3 {
        let axis = axis.normalized();
        let self_parallel_axis = axis.dot(self) * axis;
        let self_orthogonal_axis = self - self_parallel_axis;
        let self_orthogonal_rotated90 = axis.cross(self_orthogonal_axis);

        angle.cos() * self_orthogonal_axis
            + angle.sin() * self_orthogonal_rotated90
            + self_parallel_axis
    }
    pub fn rotate(&mut self, axis: Vector3, angle: Float) {
        *self = self.rotated(axis, angle);
    }
    pub fn is_finite(&self) -> bool {
        self.iter().all(|x| x.is_finite())
    }
    pub fn iter<'a>(&'a self) -> Iter<'a, Float> {
        self.0.iter()
    }
    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, Float> {
        self.0.iter_mut()
    }
    pub fn items(&self) -> &[Float; 3] {
        &self.0
    }
}

// This is bad code - though bug-free it will panic from incorrect usage.
// This is a necessary evil, as it is to my knowledge currently impossible
// to implement TryFrom<I: Iterator<_>> due to a conflicting implementation
// in core. This is very sad.
impl FromIterator<Float> for Vector3 {
    fn from_iter<I: IntoIterator<Item = Float>>(iter: I) -> Vector3 {
        let mut iterator = iter.into_iter();
        let mut result = Vector3::zero();
        for i in 0..3 {
            match iterator.next() {
                None => panic!("Cannot convert iterator of length <3 to Vector3"),
                Some(val) => result.0[i] = val,
            }
        }
        if iterator.next() != None {
            panic!("Cannot convert iterator of length >3 to Vector3")
        }
        result
    }
}

impl std::iter::Sum<Vector3> for Vector3 {
    fn sum<I: Iterator<Item = Vector3>>(iter: I) -> Vector3 {
        iter.fold(Vector3::zero(), |acc, v| acc + v)
    }
}

impl std::fmt::Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{:.2e}, {:.2e}, {:.2e}]",
            self.0[0], self.0[1], self.0[2]
        )
    }
}

// region Vector3<N> {AddAssign<Self>, SubAssign<Self>, MulAssign<Float>, DivAssign<Float>} impls
impl AddAssign for Vector3 {
    fn add_assign(&mut self, other: Self) {
        for (selfref, val) in self.iter_mut().zip(other.iter()) {
            *selfref += val;
        }
    }
}
impl SubAssign for Vector3 {
    fn sub_assign(&mut self, other: Self) {
        for (selfref, val) in self.iter_mut().zip(other.iter()) {
            *selfref -= val;
        }
    }
}

impl MulAssign<Float> for Vector3 {
    fn mul_assign(&mut self, scalar: Float) {
        for item in self.iter_mut() {
            *item *= scalar;
        }
    }
}
impl DivAssign<Float> for Vector3 {
    fn div_assign(&mut self, scalar: Float) {
        for item in self.iter_mut() {
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
