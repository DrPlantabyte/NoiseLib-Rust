pub trait CoordinateRandom2D {
	fn value_at(&self, x: i64, y: i64) -> f64; // NOTE: functions of public traits are automaticaly public
}
pub trait CoordinateRandom3D {
	fn value_at(&self, x: i64, y: i64, z: f64) -> f64;
}
pub trait CoordinateRandomND {
	fn value_at(&self, coord: &[i64]) -> f64;
}

pub struct LCGCoordinateRandomND{
	seed: u64,
}
mod lcg_funcs{
	const PRIME1: u64 = 0x5DEECE66D;
	const PRIME2: u64 = 0xB;
	const MASK: u64 = (1u64 << 48) - 1;
	const F64_MASK: u64 = 0x3FFFFFFFFFFF;
	const U64_TO_F64_MULTIPLIER: f64 = 1./(F64_MASK as f64);
	pub fn next(x: u64) -> u64 {
		let x2 = ((x * PRIME1) + PRIME2) & MASK;
		let x3 = ((x2 * PRIME1) + PRIME2) & MASK;
		return ((x2 & 0xFFFFFFFF) << 32) | (x3 & 0xFFFFFFFF);
	}
	pub fn combine(a: u64, b: u64) -> u64{
		return next(next(a) ^ next(b));
	}
	pub fn to_f64(random_long: u64) -> f64{
		return ((random_long >> 7) & F64_MASK) as f64 * U64_TO_F64_MULTIPLIER;
	}
}
impl LCGCoordinateRandomND {
	pub fn new(seed: u64) -> Self{
		Self{
			seed
		}
	}
}
impl CoordinateRandomND for LCGCoordinateRandomND {
	fn value_at(&self, coord: &[i64]) -> f64{
		use lcg_funcs::*;
		let mut x = combine(self.seed, coord[0] as u64);
		for i in 1..coord.len() {
			x = combine(x,coord[i] as u64);
		}
		return to_f64(x);
	}
}

