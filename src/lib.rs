pub fn test1() -> f64{
    return 1.0;
}

#[cfg(test)]
mod unit_tests {
    #[test]
    fn unit_test1() {
        assert_eq!(super::test1(), 42.0);
        panic!("Not implemented yet");
    }
}
