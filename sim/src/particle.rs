use crate::{math::Vec2, Parameters, WORLD_SIZE};
use rand::{rngs::SmallRng, Rng};

#[derive(Clone, Debug)]
pub struct Particle {
	pub pos: Vec2,
	pub vel: Vec2,
	pub rad: f32,
	pub kind: u16,
}

impl Particle {
	/// Generate a particle with random proprieties
	pub fn random(para: &Parameters, rng: &mut SmallRng, kinds: u16) -> Particle {
		let x = rng.gen_range(0.0..WORLD_SIZE);
		let y = rng.gen_range(0.0..WORLD_SIZE);
		let r = rng.gen_range(para.radius.clone());
		let k = rng.gen_range(0..kinds);
		Particle {
			pos: Vec2::new(x, y),
			vel: Vec2::zero(),
			rad: r,
			kind: k,
		}
	}

	// CONSIDER: rather than snapping to walls, calculate how far of they
	// should have bounced
	/// Takes the velocity of the particle and updates performing any collisions
	/// need, such was with the wall or wrapping around the space.
	pub fn update(&mut self, para: &Parameters) {
		self.pos += self.vel;
		self.vel *= 1.0 - para.friction;

		if para.wrap {
			// Wrap particle around space
			let p = &mut self.pos;
			// X component
			if p.x < 0.0 {
				p.x = WORLD_SIZE;
			} else if p.x > WORLD_SIZE {
				p.x = 0.0;
			}
			// Y component
			if p.y < 0.0 {
				p.y = WORLD_SIZE;
			} else if p.y > WORLD_SIZE {
				p.y = 0.0;
			}
		} else {
			// Collide particles with walls
			let p = &mut self.pos;
			let v = &mut self.vel;
			let r = self.rad;
			// Increased friction for collision with walls
			let f = 1.0 - (para.friction * 4.0);
			// X component
			if p.x < r {
				v.x = -v.x * f;
				p.x = r;
			} else if p.x > WORLD_SIZE - r {
				v.x = -v.x * f;
				p.x = WORLD_SIZE - r;
			}
			// Y component
			if p.y < r {
				v.y = -v.y * f;
				p.y = r;
			} else if p.y > WORLD_SIZE - r {
				v.y = -v.y * f;
				p.y = WORLD_SIZE - r;
			}
		}
	}
}
