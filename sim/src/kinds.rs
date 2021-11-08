use crate::{particle::Particle, Parameters};
use rand::rngs::SmallRng;

/// Properties of a kind of particle
#[derive(Debug)]
pub struct ParticleKind {
	/// Colour of a particle as represented as HSV
	pub colour: (u8, u8, u8),
	attract: Vec<f32>,
}

/// A managed list of all particle types
pub struct Kinds(Vec<ParticleKind>);

impl Kinds {
	/// Creates a new empty `Kinds` type ready to filled with new particle kinds to be managed
	pub fn new() -> Kinds {
		Kinds(Vec::new())
	}

	/// Using the provided `Parameters` and `SmallRng` creates a new particle kind within the
	/// expected values and appends it to the end of the list. Also returns the new index in
	/// which the particle was inserted
	pub fn new_kind(&mut self, _para: &Parameters, _rng: SmallRng) -> u16 {
		unimplemented!();
	}

	/// Remove a kind of particle from the system, this also requires a list of relevant
	/// particles to make sure that all particles of that kind are removed as well
	pub fn remove_kind(&mut self, id: u16, _parts: &mut Vec<Particle>) {
		assert!((id as usize) < self.0.len());
		unimplemented!()
	}

	/// Clears all kinds and particles without freeing any resources
	pub fn clear(&mut self, parts: &mut Vec<Particle>) {
		self.0.clear();
		parts.clear();
	}
}

impl core::ops::Index<u16> for Kinds {
	type Output = ParticleKind;
	/// Get an index into the list of particle kinds using the ID associated with it
	///
	/// ```
	/// let p = world.particles[0];
	/// let kind: ParticleKind = kinds[p.kind];
	/// println!("{:?}", kind);
	/// ```
	fn index(&self, idx: u16) -> &ParticleKind {
		&self.0[idx as usize]
	}
}
