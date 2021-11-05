#[derive(Clone, Copy, Debug)]
pub struct Vec2 {
	pub x: f32,
	pub y: f32,
}

impl Vec2 {
	pub const fn new(x: f32, y: f32) -> Vec2 {
		Vec2 { x, y }
	}
	pub const fn zero() -> Vec2 {
		Vec2::new(0.0, 0.0)
	}
	pub fn abs(&self) -> Vec2 {
		Vec2::new(self.x.abs(), self.y.abs())
	}
	pub fn trans_along(&self, ang: f32, mag: f32) -> Self {
		let d = Vec2::new(ang.cos(), ang.sin()) * mag;
		*self + d
	}
}

impl core::ops::Add for Vec2 {
	type Output = Vec2;
	fn add(self, rhs: Self) -> Vec2 {
		Vec2::new(self.x + rhs.x, self.y + rhs.y)
	}
}

impl core::ops::Add<f32> for Vec2 {
	type Output = Vec2;
	fn add(self, scalar: f32) -> Vec2 {
		Vec2::new(self.x + scalar, self.y + scalar)
	}
}

impl core::ops::Mul<f32> for Vec2 {
	type Output = Vec2;
	fn mul(self, scalar: f32) -> Vec2 {
		Vec2::new(self.x * scalar, self.y * scalar)
	}
}
