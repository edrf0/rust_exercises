use std::ops::{Add, Mul, Index, Neg, AddAssign, MulAssign, IndexMut};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2D {
    pub x: f64,
    pub y: f64,
}

impl Vector2D {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

// 1. Implement Add for Vector2D to allow v1 + v2
impl Add for Vector2D {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// 2. Implement Mul<f64> for Vector2D to allow vector scaling (e.g., v * 2.0)
impl Mul<f64> for Vector2D {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        Self {
            x: scalar * self.x,
            y: scalar * self.y,
        }
    }
}

// 3. Implement Index<usize> to allow v[0] for x and v[1] for y
impl Index<usize> for Vector2D {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("index out of range"),
        }
    }
}

// 1. Implement Neg for Vector2D to allow `-v`
impl Neg for Vector2D {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

// 2. Implement AddAssign for Vector2D to allow `v1 += v2`
impl AddAssign for Vector2D {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

// 3. Implement MulAssign<f64> for Vector2D to allow `v *= scalar`
impl MulAssign<f64> for Vector2D {
    fn mul_assign(&mut self, scalar: f64) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

// 4. Implement IndexMut<usize> to allow mutating elements via `v[0] = val`
impl IndexMut<usize> for Vector2D {
    // type Output defined in Index<usize> trait
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("index out of range"),
        }
    }
}

fn main() {
    let v1 = Vector2D::new(1.0, 2.0);
    let v2 = Vector2D::new(3.0, 4.0);
    let v3 = v1 + v2;
    println!("Sum vector: {:?}", v3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_operators() {
        let v1 = Vector2D::new(1.5, 2.0);
        let v2 = Vector2D::new(2.0, 3.5);

        // Test Add (+)
        let sum = v1 + v2;
        assert_eq!(sum, Vector2D::new(3.5, 5.5));

        // Test Mul (*) with scalar
        let scaled = v1 * 2.0;
        assert_eq!(scaled, Vector2D::new(3.0, 4.0));

        // Test Index ([])
        assert_eq!(v1[0], 1.5);
        assert_eq!(v1[1], 2.0);
    }

    #[test]
    fn test_advanced_ops() {
        let mut v1 = Vector2D::new(1.0, -2.0);

        // Test Neg (-)
        let negated = -v1;
        assert_eq!(negated, Vector2D::new(-1.0, 2.0));

        // Test AddAssign (+=)
        v1 += Vector2D::new(4.0, 5.0);
        assert_eq!(v1, Vector2D::new(5.0, 3.0));

        // Test MulAssign (*=)
        v1 *= 2.0;
        assert_eq!(v1, Vector2D::new(10.0, 6.0));

        // Test IndexMut ([])
        v1[0] = 42.0;
        assert_eq!(v1[0], 42.0);
    }
}