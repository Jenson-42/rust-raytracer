use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

use num_traits::{Float, FloatConst};
use rand::{distributions::uniform::SampleUniform, thread_rng, Rng};

#[derive(Debug, PartialEq)]
pub struct Vec3<T>(pub T, pub T, pub T);

impl<T: Clone> Clone for Vec3<T> {
    fn clone(&self) -> Self {
        Self::new(self.0.clone(), self.1.clone(), self.2.clone())
    }
}

impl<T: Copy> Copy for Vec3<T> {}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self(x, y, z)
    }
}

impl<T: SampleUniform + Float + DivAssign + FloatConst> Vec3<T> {
    pub fn random_in_unit_sphere() -> Self {
        let mut rng = thread_rng();
        let mut x = rng.gen_range(T::zero()..T::one());
        let mut y = rng.gen_range(T::zero()..T::one());
        let mut z = rng.gen_range(T::zero()..T::one());

        let mag = (x * x + y * y + z * z).sqrt();

        x /= mag;
        y /= mag;
        z /= mag;

        let u = rng.gen_range(T::zero()..T::one());
        let c = u.cbrt();

        Self::new(x * c, y * c, z * c)
    }

    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().unit_vector()
    }

    pub fn random_in_unit_disk() -> Self {
        let mut rng = thread_rng();

        let r = rng.gen_range(T::zero()..T::one()).sqrt();
        let theta = rng.gen_range(T::zero()..T::one()) * (T::one() + T::one()) * T::PI();

        Self::new(r * theta.cos(), r * theta.sin(), T::zero())
    }
}

impl<T: Default> Default for Vec3<T> {
    fn default() -> Self {
        Self::new(T::default(), T::default(), T::default())
    }
}

impl<T: Neg<Output = T>> Neg for Vec3<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.0, -self.1, -self.2)
    }
}

impl<T: Add<Output = T>> Add for Vec3<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl<T: AddAssign> AddAssign for Vec3<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl<T: AddAssign + Copy> AddAssign<T> for Vec3<T> {
    fn add_assign(&mut self, rhs: T) {
        self.0 += rhs;
        self.1 += rhs;
        self.2 += rhs;
    }
}

impl<T: Sub<Output = T>> Sub for Vec3<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl<T: Mul<Output = T>> Mul for Vec3<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Vec3<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl<T: MulAssign> MulAssign for Vec3<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
        self.2 *= rhs.2;
    }
}

impl<T: MulAssign + Copy> MulAssign<T> for Vec3<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Copy> Vec3<T> {
    pub fn length_squared(&self) -> T {
        (self.0 * self.0) + (self.1 * self.1) + (self.2 * self.2)
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Float + Copy> Vec3<T> {
    pub fn length(&self) -> T {
        self.length_squared().sqrt()
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Div<Output = T> + Float + Copy + Clone> Vec3<T> {
    pub fn unit_vector(self) -> Self {
        self / self.length()
    }
}

impl<T: Div<Output = T>> Div for Vec3<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}

impl<T: Div<Output = T> + Copy> Div<T> for Vec3<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self::new(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Copy + Float> Vec3<T> {
    pub fn dot(&self, other: &Self) -> T {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn reflect(&self, other: &Self) -> Self {
        self.to_owned() - (other.to_owned() * self.dot(other) * (T::one() + T::one())).to_owned()
    }

    pub fn refract(&self, other: &Self, etai_over_etat: T) -> Self {
        let cos_theta = T::min((-self.to_owned()).dot(other), T::one());
        let r_out_perp = (*self + *other * cos_theta) * etai_over_etat;
        let r_out_parallel = *other * -(T::abs(T::one() - r_out_perp.length_squared()).sqrt());

        r_out_perp + r_out_parallel
    }
}

impl<T: Sub<Output = T> + Mul<Output = T> + Copy> Vec3<T> {
    pub fn cross(&self, other: &Self) -> Self {
        Vec3::new(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }
}
