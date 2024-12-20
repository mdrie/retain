pub fn retain<'a>(input: &Vec<&'a str>) -> Vec<&'a str> {
    Vec::clone(input)
}

#[test]
fn keep_first_and_last() {
    let result = retain(&vec!["a", "b", "c"]);
    assert_eq!(*(result.first().unwrap_or(&"")), "a");
    assert_eq!(*(result.last().unwrap_or(&"")), "c");
}
