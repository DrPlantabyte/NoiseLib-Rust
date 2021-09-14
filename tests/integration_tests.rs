use std::fs;
use std::env;

use image::*;
use rand::*;

use noiselib;

pub fn setup() {
	let test_dir = env!("CARGO_TARGET_TMPDIR");
	fs::create_dir_all(test_dir).unwrap();
	env::set_current_dir(test_dir).unwrap();
}

#[test]
fn test_2d_noise() {
	setup();
	println!("{}", noiselib::test1());
	let seed: u64 = random();
	// let coord_prng = noiselib::rand::LCGCoordinateRandom2D::new(seed);
	// let frequency = 1./11.; // grid-spacing of 11
	// let noise_gen = noiselib::FractalNoiseNoiseGenerator2D::new(coord_prng, frequency);
	// let size = 512;
	// let mut img: RgbImage = ImageBuffer::new(size, size);
	// for py in 0..size{
	// 	for px in 0..size{
	// 		let v: u8 = (noise_gen.get_value(1., px as f64, py as f64) * 255) as u8;
	// 		let v = v.max(0).min(255);
	// 		let pixel = Rgb([v,v,v]);
	// 		img.put_pixel(px, py, pixel);
	// 	}
	// }
	// img.save("test_2d_noise.png").unwrap();

}