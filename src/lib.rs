
mod rand;


pub fn test1() -> f64{
	return 1.0;
}

#[cfg(test)]
mod unit_tests {
	#[test]
	fn unit_test1() {
		assert_eq!(super::test1(), 42.0);
		// panic!("Not implemented yet");
	}
	#[test]
	fn test_LCGCoordinateRandomND() {
		let crng = rand::LCGCoordinateRandomND::new(1);
		println!("Value at {}", crng.value_at(&[1,2,3,4]))
		// assert_eq!(super::test1(), 42.0);
		// panic!("Not implemented yet");
	}
}
