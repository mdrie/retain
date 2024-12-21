use algo::to_be_deleted;
use chrono::DateTime;

mod algo;

pub fn retain<'a>(input: &Vec<&'a str>) -> Vec<&'a str> {
    let test = DateTime::parse_from_rfc3339("2023-12-03T13:03:12Z").unwrap().to_utc().timestamp();
    Vec::clone(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keep_first_and_last() {
        let result = retain(&vec!["a", "b", "c"]);
        assert_eq!(*(result.first().unwrap_or(&"")), "a");
        assert_eq!(*(result.last().unwrap_or(&"")), "c");
    }

    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_arb(input in prop::collection::vec("(2022|2023|2024)-0[1-9]", 0..10)) {
            let inp:Vec<&str> = input.iter().map(|s| s.as_str()).collect();
            let result = retain(&inp);
            println!("{:?}", result);
        }
    }
}
