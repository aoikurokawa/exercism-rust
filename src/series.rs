pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut expected = vec![];
    let mut digits = digits;

    while digits.len() >= len {
        let mut digit = String::new();
        for i in 0..len {
            let ch = digits.as_bytes()[i];
            digit.push(ch as char);
        }
        expected.push(digit.to_string());

        if !digits.is_empty() {
            let (_first, second) = digits.split_at(1);
            digits = second;
        } else {
            break;
        }
    }

    expected
}

#[cfg(test)]
mod tests {
    use crate::series::*;

    #[test]
    fn test_with_zero_length() {
        let expected = vec!["".to_string(); 6];
        assert_eq!(series("92017", 0), expected);
    }

    #[test]
    fn test_with_length_2() {
        let expected = vec![
            "92".to_string(),
            "20".to_string(),
            "01".to_string(),
            "17".to_string(),
        ];
        assert_eq!(series("92017", 2), expected);
    }

    #[test]
    fn test_with_numbers_length() {
        let expected = vec!["92017".to_string()];
        assert_eq!(series("92017", 5), expected);
    }

    #[test]
    fn test_too_long() {
        let expected: Vec<String> = vec![];
        assert_eq!(series("92017", 6), expected);
    }

    #[test]
    fn test_way_too_long() {
        let expected: Vec<String> = vec![];
        assert_eq!(series("92017", 42), expected);
    }
}
