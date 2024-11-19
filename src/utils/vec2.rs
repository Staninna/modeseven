//! A 2D vector implementation

/// A two-dimensional vector with float components
///
/// Vec2 provides common vector operations needed for:
/// * Game physics calculations
/// * Position and velocity tracking
/// * Direction and rotation math
/// * Basic vector arithmetic
///
/// All operations use single-precision floating point math.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    /// X component of the vector
    pub x: f32,
    /// Y component of the vector  
    pub y: f32,
}

impl Vec2 {
    /// Creates a new vector with given components
    ///
    /// # Arguments
    ///
    /// * `x` - X component value
    /// * `y` - Y component value
    ///
    /// # Returns
    ///
    /// A new Vec2 with the specified components
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Creates a zero vector (0, 0)
    ///
    /// # Returns
    ///
    /// A new Vec2 with both components set to 0.0
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    /// Calculates the vector's length using √(x² + y²)
    ///
    /// # Returns
    ///
    /// The vector's magnitude as a float
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Returns this vector with length 1.0
    ///
    /// Returns the original vector if length is zero
    ///
    /// # Returns
    ///
    /// A new unit-length Vec2 in the same direction
    pub fn normalized(&self) -> Self {
        let len = self.length();
        if len > 0.0 {
            Self {
                x: self.x / len,
                y: self.y / len,
            }
        } else {
            *self
        }
    }

    /// Rotates the vector by an angle
    ///
    /// # Arguments
    ///
    /// * `angle` - Rotation in radians (counterclockwise)
    ///
    /// # Returns
    ///
    /// A new Vec2 rotated around the origin
    pub fn rotate(&self, angle: f32) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self {
            x: self.x * cos - self.y * sin,
            y: self.x * sin + self.y * cos,
        }
    }
}

impl std::ops::Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::Div<f32> for Vec2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl std::ops::Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl std::ops::Rem<f32> for Vec2 {
    type Output = Self;

    fn rem(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x % rhs,
            y: self.y % rhs,
        }
    }
}

impl std::ops::AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl std::ops::MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl std::ops::DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl std::ops::RemAssign<f32> for Vec2 {
    fn rem_assign(&mut self, rhs: f32) {
        self.x %= rhs;
        self.y %= rhs;
    }
}
