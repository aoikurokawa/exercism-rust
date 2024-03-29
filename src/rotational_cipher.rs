fn hail(c: char, shift: u8) -> char {
    if !c.is_ascii_alphabetic() {
        return c;
    }
    let start = if c.is_ascii_uppercase() { b'A' } else { b'a' };
    (((c as u8 - start) + shift) % 26 + start) as char
}

pub fn rotate(input: &str, key: i8) -> String {
    let key = if key < 0 { (26 + key) as u8 } else { key as u8 };
    input.chars().map(|c| hail(c, key)).collect::<String>()
}

#[cfg(test)]
mod tests {
    use crate::rotational_cipher as cipher;

    #[test]
    fn rotate_a_1() {
        assert_eq!("b", cipher::rotate("a", 1));
    }

    #[test]
    fn rotate_a_26() {
        assert_eq!("a", cipher::rotate("a", 26));
    }

    #[test]
    fn rotate_a_0() {
        assert_eq!("a", cipher::rotate("a", 0));
    }

    #[test]
    fn rotate_m_13() {
        assert_eq!("z", cipher::rotate("m", 13));
    }

    #[test]
    fn rotate_n_13_with_wrap() {
        assert_eq!("a", cipher::rotate("n", 13));
    }

    #[test]
    fn rotate_caps() {
        assert_eq!("TRL", cipher::rotate("OMG", 5));
    }

    #[test]
    fn rotate_spaces() {
        assert_eq!("T R L", cipher::rotate("O M G", 5));
    }

    #[test]
    fn rotate_numbers() {
        assert_eq!(
            "Xiwxmrk 1 2 3 xiwxmrk",
            cipher::rotate("Testing 1 2 3 testing", 4)
        );
    }

    #[test]
    fn rotate_punctuation() {
        assert_eq!(
            "Gzo\'n zvo, Bmviyhv!",
            cipher::rotate("Let\'s eat, Grandma!", 21)
        );
    }

    #[test]
    fn rotate_all_the_letters() {
        assert_eq!(
            "Gur dhvpx oebja sbk whzcf bire gur ynml qbt.",
            cipher::rotate("The quick brown fox jumps over the lazy dog.", 13)
        );
    }

    #[test]
    fn rotate_m_negative_1() {
        assert_eq!("l", cipher::rotate("m", -1));
    }

    #[test]
    fn rotate_letters_negative_26() {
        assert_eq!("omg", cipher::rotate("omg", -26));
    }
}
