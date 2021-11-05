use core::ops::{Range, RangeInclusive};

use rand::{rngs::SmallRng, Rng, SeedableRng};
use rand_distr::{Alphanumeric, Distribution, Normal};
use rand_seeder::Seeder;

mod math;
use math::Vec2;

use rayon::prelude::*;

/// Flattened arrays of how each particle interacts with each other
pub struct ParticleKinds {
	kinds: usize,
	colours: Vec<(u8, u8, u8)>,
	attrac: Vec<f32>,
	range: Vec<f32>,
}

impl ParticleKinds {
	pub fn new(kinds: usize) -> ParticleKinds {
		ParticleKinds {
			kinds: kinds,
			colours: vec![(0, 0, 0); kinds],
			attrac: vec![0.0; kinds * kinds],
			range: vec![0.0; kinds * kinds],
		}
	}
	pub fn clear(&mut self) {
		for a in self.attrac.iter_mut() {
			*a = 0.0;
		}
		for a in self.range.iter_mut() {
			*a = 0.0;
		}
	}
	pub fn colour(&self, a: usize) -> (u8, u8, u8) {
		self.colours[a]
	}
	pub fn set_colour(&mut self, a: usize, v: (u8, u8, u8)) {
		self.colours[a] = v;
	}
	pub fn attrac(&self, a: usize, b: usize) -> f32 {
		let i = a * self.kinds + b;
		self.attrac[i]
	}
	pub fn set_attrac(&mut self, a: usize, b: usize, v: f32) {
		let i = a * self.kinds + b;
		self.attrac[i] = v;
	}
	pub fn range(&self, a: usize, b: usize) -> f32 {
		let i = a * self.kinds + b;
		self.range[i]
	}
	pub fn set_range(&mut self, a: usize, b: usize, v: f32) {
		let i = a * self.kinds + b;
		self.range[i] = v;
	}
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
	let f = h * 6.0 - (h * 6.0).floor();
	let p = v * (1.0 - s);
	let q = v * (1.0 - f * s);
	let t = v * (1.0 - (1.0 - f) * s);

	let (r, g, b) = match (h * 6.0).floor() as i32 % 6 {
		0 => (v, t, p),
		1 => (q, v, p),
		2 => (p, v, t),
		3 => (p, q, v),
		4 => (t, p, v),
		5 => (v, p, q),
		_ => unreachable!(),
	};

	((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
}

#[derive(Debug, Clone)]
pub struct Particle {
	/// Position of the vector
	pub pos: Vec2,
	/// Current velocity of the particle
	pub vel: Vec2,
	/// Radius of the particle
	pub rad: f32,
	/// What kind the particle is, how it interacts
	pub kind: usize,
}

impl Particle {
	pub fn update(&mut self, params: &WorldParameters) {
		// Apply velocity and friction
		self.pos = self.pos + self.vel;
		self.vel = self.vel * (1.0 - params.friction);

		// // Wrap particle around space
		// let pos = &mut self.pos;
		// if pos.x < 0.0 {
		// 	pos.x = params.bounds.0;
		// } else if pos.x >= params.bounds.0 {
		// 	pos.x = 0.0;
		// }
		// if pos.y < 0.0 {
		// 	pos.y = params.bounds.1;
		// } else if pos.y >= params.bounds.1 {
		// 	pos.y = 0.0;
		// }

		// Collide with walls
		let pos = &mut self.pos;
		let vel = &mut self.vel;
		let dia = self.rad * self.rad;
		let fric = 1.0 - (params.friction * 4.0);
		if pos.x <= dia {
			vel.x = -vel.x * fric;
			pos.x = dia;
		} else if pos.x >= params.bounds.0 - dia {
			vel.x = -vel.x * fric;
			pos.x = params.bounds.0 - dia;
		}
		if pos.y <= dia {
			vel.y = -vel.y * fric;
			pos.y = dia;
		} else if pos.y >= params.bounds.1 - dia {
			vel.y = -vel.y * fric;
			pos.y = params.bounds.1 - dia;
		}
	}
}

#[derive(Clone)]
pub struct WorldParameters {
	/// The size of the world (w, h)
	pub bounds: (f32, f32),
	/// Average mean attraction power
	pub attrac_mean: f32,
	/// Attraction power standard deviation
	pub attrac_dev: f32,
	/// Range attraction distance
	pub attrac_range: Range<f32>,
	/// Radius range
	pub radius_range: RangeInclusive<f32>,
	/// Friction of particles
	pub friction: f32,
	/// World seed
	pub seed: String,
}

pub struct World {
	pub parts: Vec<Particle>,
	params: WorldParameters,
	pub kinds: ParticleKinds,
	rng: SmallRng,
}

impl World {
	pub fn new(count: usize, types: usize, params: WorldParameters) -> World {
		let mut world = World {
			parts: Vec::with_capacity(count),
			params: params,
			kinds: ParticleKinds::new(types),
			rng: SmallRng::from_rng(rand::thread_rng()).unwrap(),
		};
		world.reseed(world.params.clone());
		world
	}

	pub fn reseed(&mut self, mut params: WorldParameters) {
		let count = self.parts.capacity();
		self.parts.clear();
		self.kinds.clear();
		if params.seed.is_empty() {
			params.seed = generate_seed();
		}
		println!("Seed: \"{}\"", params.seed);
		self.rng = Seeder::from(&params.seed).make_rng();
		self.params = params;

		self.seed_types();
		for _ in 0..count {
			let part = self.new_particle();
			self.parts.push(part);
		}
	}

	pub fn add_particle(&mut self) {
		let part = self.new_particle();
		self.parts.push(part);
	}

	pub fn del_particle(&mut self) {
		let _ = self.parts.pop();
	}

	fn new_particle(&mut self) -> Particle {
		let rng = &mut self.rng;
		let x = rng.gen_range(0.0..self.params.bounds.0);
		let y = rng.gen_range(0.0..self.params.bounds.1);
		let r = rng.gen_range(self.params.radius_range.clone());
		let k = rng.gen_range(0..self.kinds.kinds);
		Particle {
			pos: Vec2::new(x, y),
			vel: Vec2::zero(),
			rad: r,
			kind: k,
		}
	}

	fn seed_types(&mut self) {
		let kinds = &mut self.kinds;
		let rng = &mut self.rng;
		let dist = Normal::new(self.params.attrac_mean, self.params.attrac_dev).unwrap();

		for i in 0..kinds.kinds {
			let col = hsv_to_rgb(rng.gen(), 1.0, 1.0);
			kinds.set_colour(i, col);
			for j in 0..kinds.kinds {
				let a = if i == j {
					-dist.sample(rng).abs()
				} else {
					dist.sample(rng)
				};
				kinds.set_attrac(i, j, a);
				kinds.set_range(i, j, rng.gen_range(self.params.attrac_range.clone()));
			}
		}
	}

	pub fn influence(&self, a: &mut Particle) {
		let others = &self.parts;
		let mut neighbours = 0;
		// let p = &self.params;
		for b in others.iter() {
			// Get difference between positions
			let mut dx = a.pos.x - b.pos.x;
			let mut dy = a.pos.y - b.pos.y;

			// // Wrap distance within the world to smallest value
			// if dx > p.bounds.0 / 2.0 {
			// 	dx -= p.bounds.0;
			// } else if dx < -p.bounds.0 / 2.0 {
			// 	dx += p.bounds.0;
			// }
			// if dy > p.bounds.1 / 2.0 {
			// 	dy -= p.bounds.1;
			// } else if dy < -p.bounds.1 / 2.0 {
			// 	dy += p.bounds.1;
			// }

			// Get the range which interaction can happen
			let r = self.kinds.range(a.kind, b.kind);

			// Get distance squared
			let dis2 = dx * dx + dy * dy;
			// If outside of area of influence continue to next particle
			if dis2 > r * r || dis2 < 0.01 {
				continue;
			}

			// Normalise the distance so we get an "angle" to the particle
			let dis = f32::sqrt(dis2);
			dx /= dis;
			dy /= dis;

			if dis < 20.0 && a.kind == b.kind {
				neighbours += 1;
			}

			// Calculate and apply forces
			let mut f;
			let att = self.kinds.attrac(a.kind, b.kind) + self.kinds.attrac(b.kind, a.kind);
			if dis > a.rad * 2.0 + b.rad * 2.0 {
				f = (1.0 / dis2) * att;
			} else {
				f = (1.0 / dis2) * -att;
			}
			f = f.min(1.0);

			a.vel = a.vel + Vec2::new(dx, dy) * f;
		}
		if neighbours >= 4 {
			a.kind = rand::thread_rng().gen_range(0..self.kinds.kinds);
		}
	}

	pub fn step(&mut self) {
		let threads = num_cpus::get();

		let chunk_size = (self.parts.len() as f32 / threads as f32).floor() as usize;

		let mut new_parts = self.parts.clone();

		new_parts.par_chunks_mut(chunk_size).for_each(|parts| {
			for point in parts {
				self.influence(point);
				point.update(&self.params);
			}
		});

		self.parts = new_parts;
	}
}

pub fn generate_seed() -> String {
	let mut rng = SmallRng::from_rng(rand::thread_rng()).unwrap();
	std::iter::repeat(())
		.map(|_| rng.sample(Alphanumeric))
		.map(char::from)
		.take(8)
		.collect()
}
