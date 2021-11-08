macro_rules! vec2 {
	($n:ident, $t:ident) => {
		/// A set of two coordinates which can be interpreted as a point in space or vector in 2D space.
		#[derive(Clone, Copy, Debug, Default, PartialEq)]
		pub struct $n {
			pub x: $t,
			pub y: $t,
		}

		impl $n {
			#[inline]
			pub const fn new(x: $t, y: $t) -> Self {
				Self { x, y }
			}

			/// Fills the X & Y components of the vector with the same value
			#[inline]
			pub const fn rep(v: $t) -> Self {
				Self::new(v, v)
			}

			#[inline]
			pub const fn zero() -> Self {
				Self::rep(0.0)
			}

			#[inline]
			pub const fn one() -> Self {
				Self::rep(1.0)
			}

			#[inline]
			pub fn abs(&self) -> Self {
				Self::new(self.x.abs(), self.y.abs())
			}

			/// The magnitude of the vector squared, for proper magnitude use `.mag()`
			#[inline]
			pub fn mag_sq(&self) -> $t {
				self.x * self.x + self.y * self.y
			}

			/// Returns the magnitude of the vector
			#[inline]
			pub fn mag(&self) -> $t {
				self.mag_sq().sqrt()
			}

			/// Normalises the components of a vector in place
			#[inline]
			pub fn normalise(&mut self) {
				let mag = self.mag();
				self.x *= mag;
				self.y *= mag;
			}

			/// Returns a normalised copy of the vector
			#[inline]
			#[must_use = "Perhaps you meant to use `.normalise()`?"]
			pub fn normalised(&self) -> Self {
				let mut s = self.clone();
				s.normalise();
				s
			}

			/// Applies a function onto both of the components of the vector
			///
			/// ```
			/// let mut v = Vec2::rep(2.0);
			/// v.apply(|c| {
			/// 	c * 2.0
			/// });
			/// assert_eq!(Vec2::rep(4.0), v);
			/// ```
			#[inline]
			pub fn apply<F: FnMut(f32) -> f32>(&mut self, mut f: F) {
				self.x = f(self.x);
				self.y = f(self.y);
			}
		}

		impl core::ops::Add for $n {
			type Output = Self;
			#[inline]
			fn add(self, rhs: Self) -> Self::Output {
				Self::new(self.x + rhs.x, self.y + rhs.y)
			}
		}

		impl core::ops::AddAssign for $n {
			#[inline]
			fn add_assign(&mut self, rhs: Self) {
				self.x += rhs.x;
				self.y += rhs.y;
			}
		}

		impl core::ops::Sub for $n {
			type Output = Self;
			#[inline]
			fn sub(self, rhs: Self) -> Self::Output {
				Self::new(self.x - rhs.x, self.y - rhs.y)
			}
		}

		impl core::ops::SubAssign for $n {
			#[inline]
			fn sub_assign(&mut self, rhs: Self) {
				self.x -= rhs.x;
				self.y -= rhs.y;
			}
		}

		impl core::ops::Mul for $n {
			type Output = Self;
			#[inline]
			fn mul(self, rhs: Self) -> Self::Output {
				Self::new(self.x * rhs.x, self.y * rhs.y)
			}
		}

		impl core::ops::Mul<$t> for $n {
			type Output = Self;
			#[inline]
			fn mul(self, rhs: $t) -> Self::Output {
				Self::new(self.x * rhs, self.y * rhs)
			}
		}

		impl core::ops::MulAssign for $n {
			#[inline]
			fn mul_assign(&mut self, rhs: Self) {
				self.x *= rhs.x;
				self.y *= rhs.y;
			}
		}

		impl core::ops::MulAssign<$t> for $n {
			#[inline]
			fn mul_assign(&mut self, rhs: $t) {
				self.x *= rhs;
				self.y *= rhs;
			}
		}

		impl core::ops::Div for $n {
			type Output = Self;
			#[inline]
			fn div(self, rhs: Self) -> Self::Output {
				Self::new(self.x / rhs.x, self.y / rhs.y)
			}
		}

		impl core::ops::Div<$t> for $n {
			type Output = Self;
			#[inline]
			fn div(self, rhs: $t) -> Self::Output {
				Self::new(self.x / rhs, self.y / rhs)
			}
		}

		impl core::ops::DivAssign for $n {
			#[inline]
			fn div_assign(&mut self, rhs: Self) {
				self.x /= rhs.x;
				self.y /= rhs.y;
			}
		}

		impl core::ops::DivAssign<$t> for $n {
			#[inline]
			fn div_assign(&mut self, rhs: $t) {
				self.x /= rhs;
				self.y /= rhs;
			}
		}

		impl core::ops::Neg for $n {
			type Output = Self;
			#[inline]
			fn neg(self) -> Self {
				Self::new(-self.x, -self.y)
			}
		}
	};
}

vec2!(Vec2, f32);
