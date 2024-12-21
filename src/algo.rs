use chrono::*;

pub fn to_be_deleted(found_dates: &Vec<DateTime<Utc>>) -> &Vec<DateTime<Utc>> {
    return found_dates;
}

fn to_be_deleted_timestamps(found_timestamps: &Vec<i64>) -> Vec<i64> {
    let second = found_timestamps.get(1);
    return match second {
        Some(a) => vec![*a],
        None => vec![],
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    const DAY: i64 = 60 * 60 * 24;
    const YEAR: i64 = DAY * 365;

    proptest! {
        #[test]
        fn test_arb(input in prop::collection::vec(0..5*YEAR, 0..10)) {
            let result = to_be_deleted_timestamps(&input);
            println!("{:?}   {:?}", input.iter().map(|i| (*i as f32)/(YEAR as f32)).collect::<Vec<f32>>(), result);
        }
    }
}
