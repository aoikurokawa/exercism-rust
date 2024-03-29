/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    let score_sym = |c| match c {
        'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T' => 1,
        'D' | 'G' => 2,
        'B' | 'C' | 'M' | 'P' => 3,
        'F' | 'H' | 'V' | 'W' | 'Y' => 4,
        'K' => 5,
        'J' | 'X' => 8,
        'Q' | 'Z' => 10,
        _ => 0,
    };

    word.chars()
        .filter(char::is_ascii)
        .map(|x| x.to_ascii_uppercase())
        .map(score_sym)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::scrabble_score::*;
    #[test]
    fn a_is_worth_one_point() {
        assert_eq!(score("a"), 1);
    }
    #[test]
    fn scoring_is_case_insensitive() {
        assert_eq!(score("A"), 1);
    }
    #[test]
    fn f_is_worth_four() {
        assert_eq!(score("f"), 4);
    }
    #[test]
    fn two_one_point_letters_make_a_two_point_word() {
        assert_eq!(score("at"), 2);
    }
    #[test]
    fn three_letter_word() {
        assert_eq!(score("zoo"), 12);
    }
    #[test]
    fn medium_word() {
        assert_eq!(score("street"), 6);
    }
    #[test]
    fn longer_words_with_valuable_letters() {
        assert_eq!(score("quirky"), 22);
    }
    #[test]
    fn long_mixed_case_word() {
        assert_eq!(score("OxyphenButazone"), 41);
    }
    #[test]
    fn non_english_scrabble_letters_do_not_score() {
        assert_eq!(score("pinata"), 8, "'n' should score 1");
        assert_eq!(score("piñata"), 7, "'ñ' should score 0");
    }
    #[test]
    fn empty_words_are_worth_zero() {
        assert_eq!(score(""), 0);
    }
    #[test]
    fn all_letters_work() {
        assert_eq!(score("abcdefghijklmnopqrstuvwxyz"), 87);
    }
    #[test]
    fn german_letters_do_not_score() {
        assert_eq!(score("STRASSE"), 7, "\"SS\" should score 2");
        assert_eq!(score("STRAßE"), 5, "'ß' should score 0");
    }
}
