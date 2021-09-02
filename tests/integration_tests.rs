use std::fs;
use std::env;

use noiselib;

pub fn setup() {
	let test_dir = env!("CARGO_TARGET_TMPDIR");
	fs::create_dir_all(test_dir).unwrap();
	env::set_current_dir(test_dir).unwrap();
}

#[test]
fn test_feature() {
    setup();
    println!("{}", noiselib::test1());
	
	let seed: u64 = 1234567890; 
	let rand_2d = noiselib::rand::LCGCoordinateRandom2D::new(seed);
	let wrap_dimensions = [[0.0, 3.0], [0.0, 1.5]];
	let base_frequency = 1.;
	let frequency_scalar = 2.; // double the frequency for each octave
	let base_magnitude = 1.;
	let magnitude_scalar = 0.5; // halve the amplitude for each octave
	let noise_generator = noiselib::TesselNoise2D::new(
			&rand_2d,
			base_frequency, 
			frequency_scalar,
			base_magnitude,
			magnitude_scalar,
			Some(wrap_dimensions)
	);
	
    panic!("Not implemented yet");
}