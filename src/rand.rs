pub trait CoordinateRandom2D {
	fn value_at(&self, x: i64, y: i64) -> f64;
}