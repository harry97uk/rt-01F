use std::ops::{ AddAssign, MulAssign, DivAssign, Add, Sub, Mul, Div, Neg };

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Vector3 {
    e: [f64; 3],
}

impl Vector3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Vector3 { e: [e0, e1, e2] }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length(&self) -> f64 {
        return self.length_squared().sqrt();
    }

    pub fn length_squared(&self) -> f64 {
        return self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2];
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, other: Vector3) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
    }
}

impl MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, t: f64) {
        self.e[0] *= t;
        self.e[1] *= t;
        self.e[2] *= t;
    }
}

impl DivAssign<f64> for Vector3 {
    fn div_assign(&mut self, t: f64) {
        *self *= 1.0 / t;
    }
}

pub type Point3 = Vector3;

// Vector Utility Functions

impl std::fmt::Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {
            e: [self.e[0] + other.e[0], self.e[1] + other.e[1], self.e[2] + other.e[2]],
        }
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3 {
            e: [self.e[0] - other.e[0], self.e[1] - other.e[1], self.e[2] - other.e[2]],
        }
    }
}

impl Mul for Vector3 {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Vector3 {
        Vector3 {
            e: [self.e[0] * other.e[0], self.e[1] * other.e[1], self.e[2] * other.e[2]],
        }
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, t: f64) -> Vector3 {
        Vector3 {
            e: [self.e[0] * t, self.e[1] * t, self.e[2] * t],
        }
    }
}

impl Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, v: Vector3) -> Vector3 {
        Vector3 {
            e: [self * v.e[0], self * v.e[1], self * v.e[2]],
        }
    }
}

impl Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, t: f64) -> Vector3 {
        (1.0 / t) * self
    }
}

impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Vector3 {
        Vector3 {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

pub fn dot(u: Vector3, v: Vector3) -> f64 {
    u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
}

pub fn cross(u: Vector3, v: Vector3) -> Vector3 {
    Vector3 {
        e: [
            u.e[1] * v.e[2] - u.e[2] * v.e[1],
            u.e[2] * v.e[0] - u.e[0] * v.e[2],
            u.e[0] * v.e[1] - u.e[1] * v.e[0],
        ],
    }
}

pub fn unit_vector(v: Vector3) -> Vector3 {
    v / v.length()
}
