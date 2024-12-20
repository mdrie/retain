use proptest::prelude::*;

proptest! {
    #[test]
    fn test_arb(input in prop::collection::vec("a-z", 0..10)) {
        let inp:Vec<&str> = input.iter().map(|s| s.as_str()).collect();
        let result = retain::retain(&inp);
        println!("{:?}", result);
    }
}
