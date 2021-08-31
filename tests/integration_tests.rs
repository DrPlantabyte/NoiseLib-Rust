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
    panic!("Not implemented yet");
}