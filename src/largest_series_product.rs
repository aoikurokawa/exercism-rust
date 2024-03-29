#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    let mut digits_vec: Vec<u64> = vec![];
    for ch in string_digits.chars() {
        if !ch.is_numeric() {
            return Err(Error::InvalidDigit(ch));
        }
        digits_vec.push(ch.to_digit(10).unwrap().into());
    }
    if span > digits_vec.len() {
        return Err(Error::SpanTooLong);
    }

    if span == 0 {
        return Ok(1);
    }

    let largest: u64 = digits_vec
        .windows(span)
        .map(|window| window.iter().product())
        .max()
        .unwrap_or(1);

    Ok(largest)
}

#[cfg(test)]
mod tests {
    use crate::largest_series_product::*;

    #[test]
    fn return_is_a_result() {
        assert!(lsp("29", 2).is_ok());
    }

    #[test]
    fn find_the_largest_product_when_span_equals_length() {
        assert_eq!(Ok(18), lsp("29", 2));
    }

    #[test]
    fn find_the_largest_product_of_two_with_numbers_in_order() {
        assert_eq!(Ok(72), lsp("0123456789", 2));
    }

    #[test]
    fn find_the_largest_product_of_two_with_numbers_not_in_order() {
        assert_eq!(Ok(48), lsp("576802143", 2));
    }

    #[test]
    fn find_the_largest_product_of_three_with_numbers_in_order() {
        assert_eq!(Ok(504), lsp("0123456789", 3));
    }

    #[test]
    fn find_the_largest_product_of_three_with_numbers_not_in_order() {
        assert_eq!(Ok(270), lsp("1027839564", 3));
    }

    #[test]
    fn find_the_largest_product_of_five_with_numbers_in_order() {
        assert_eq!(Ok(15_120), lsp("0123456789", 5));
    }

    #[test]
    fn span_of_six_in_a_large_number() {
        assert_eq!(
            Ok(23_520),
            lsp("73167176531330624919225119674426574742355349194934", 6)
        );
    }

    #[test]
    fn returns_zero_if_number_is_zeros() {
        assert_eq!(Ok(0), lsp("0000", 2));
    }

    #[test]
    fn returns_zero_if_all_products_are_zero() {
        assert_eq!(Ok(0), lsp("99099", 3));
    }

    #[test]
    fn a_span_is_longer_than_number_is_an_error() {
        assert_eq!(Err(Error::SpanTooLong), lsp("123", 4));
    }

    // There may be some confusion about whether this should be 1 or error.

    // The reasoning for it being 1 is this:

    // There is one 0-character string contained in the empty string.

    // That's the empty string itself.

    // The empty product is 1 (the identity for multiplication).

    // Therefore LSP('', 0) is 1.

    // It's NOT the case that LSP('', 0) takes max of an empty list.

    // So there is no error.

    // Compare against LSP('123', 4):

    // There are zero 4-character strings in '123'.

    // So LSP('123', 4) really DOES take the max of an empty list.

    // So LSP('123', 4) errors and LSP('', 0) does NOT.

    #[test]
    fn an_empty_string_and_no_span_returns_one() {
        assert_eq!(Ok(1), lsp("", 0));
    }

    #[test]
    fn a_non_empty_string_and_no_span_returns_one() {
        assert_eq!(Ok(1), lsp("123", 0));
    }

    #[test]
    fn empty_string_and_non_zero_span_is_an_error() {
        assert_eq!(Err(Error::SpanTooLong), lsp("", 1));
    }

    #[test]
    fn a_string_with_non_digits_is_an_error() {
        assert_eq!(Err(Error::InvalidDigit('a')), lsp("1234a5", 2));
    }
}
