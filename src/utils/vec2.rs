//! A 2D vector implementation optimized for game physics and graphics operations

/// A two-dimensional vector with single-precision floating point components
///
/// Vec2 provides a basic 2D vector implementation with common operations needed
/// for game physics and graphics calculations. It includes methods for vector
/// arithmetic, normalization, rotation, and length calculations.
///
/// The type implements several traits for convenience:
/// * Debug - For formatted debug output
/// * Clone, Copy - For easy value semantics
/// * PartialEq - For vector comparison
/// * Add, Sub, `Mul<f32>` - For vector arithmetic
///
/// # Example
///
/// ```rust
/// let velocity = Vec2::new(3.0, 4.0);
/// let normalized = velocity.normalized();
/// let rotated = velocity.rotate(std::f32::consts::PI / 2.0);
/// let scaled = velocity * 2.0;
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    /// X component of the vector
    pub x: f32,
    /// Y component of the vector
    pub y: f32,
}

impl Vec2 {
    /// Creates a new vector with the specified components
    ///
    /// # Arguments
    ///
    /// * `x` - The x component of the vector
    /// * `y` - The y component of the vector
    ///
    /// # Returns
    ///
    /// A new Vec2 instance with the specified components
    ///
    /// # Example
    ///
    /// ```rust
    /// let position = Vec2::new(10.0, 20.0);
    /// ```
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Creates a zero vector (0, 0)
    ///
    /// # Returns
    ///
    /// A new Vec2 instance with both components set to 0.0
    ///
    /// # Example
    ///
    /// ```rust
    /// let origin = Vec2::zero();
    /// assert_eq!(origin.x, 0.0);
    /// assert_eq!(origin.y, 0.0);
    /// ```
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    /// Calculates the length (magnitude) of the vector
    ///
    /// Uses the Pythagorean theorem to calculate the vector's length:
    /// length = √(x² + y²)
    ///
    /// # Returns
    ///
    /// The length of the vector as a floating point number
    ///
    /// # Example
    ///
    /// ```rust
    /// let vec = Vec2::new(3.0, 4.0);
    /// assert_eq!(vec.length(), 5.0);
    /// ```
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Returns a normalized (unit length) version of the vector
    ///
    /// Creates a new vector in the same direction but with length 1.0.
    /// If the vector has zero length, returns a copy of the original vector
    /// to avoid division by zero.
    ///
    /// # Returns
    ///
    /// A new Vec2 with the same direction but unit length
    ///
    /// # Example
    ///
    /// ```rust
    /// let vec = Vec2::new(3.0, 4.0);
    /// let normalized = vec.normalized();
    /// assert!((normalized.length() - 1.0).abs() < 1e-6);
    /// ```
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

    /// Rotates the vector by the specified angle
    ///
    /// Creates a new vector by rotating this vector counterclockwise
    /// by the specified angle (in radians) around the origin.
    ///
    /// Uses the 2D rotation matrix:
    /// ```text
    /// | cos θ  -sin θ |
    /// | sin θ   cos θ |
    /// ```
    ///
    /// # Arguments
    ///
    /// * `angle` - The rotation angle in radians (counterclockwise)
    ///
    /// # Returns
    ///
    /// A new Vec2 representing the rotated vector
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::f32::consts::PI;
    /// let vec = Vec2::new(1.0, 0.0);
    /// let rotated = vec.rotate(PI / 2.0);  // 90 degrees
    /// assert!((rotated.x - 0.0).abs() < 1e-6);
    /// assert!((rotated.y - 1.0).abs() < 1e-6);
    /// ```
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

    /// Adds two vectors component-wise
    ///
    /// # Example
    ///
    /// ```rust
    /// let v1 = Vec2::new(1.0, 2.0);
    /// let v2 = Vec2::new(3.0, 4.0);
    /// let sum = v1 + v2;
    /// assert_eq!(sum, Vec2::new(4.0, 6.0));
    /// ```
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Mul<f32> for Vec2 {
    type Output = Self;

    /// Multiplies a vector by a scalar
    ///
    /// # Example
    ///
    /// ```rust
    /// let vec = Vec2::new(2.0, 3.0);
    /// let scaled = vec * 2.0;
    /// assert_eq!(scaled, Vec2::new(4.0, 6.0));
    /// ```
    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Self;

    /// Subtracts two vectors component-wise
    ///
    /// # Example
    ///
    /// ```rust
    /// let v1 = Vec2::new(4.0, 6.0);
    /// let v2 = Vec2::new(1.0, 2.0);
    /// let diff = v1 - v2;
    /// assert_eq!(diff, Vec2::new(3.0, 4.0));
    /// ```
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
