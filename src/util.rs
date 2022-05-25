fn parse_digit(digit: &str) -> Option<i32> {
    match digit {
        "zero" => Some(0),
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        &_ => None
    }
}

#[test]
fn test_parse_digit() {
    assert!(parse_digit("two") == Some(2));
}
