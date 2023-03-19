use std::collections::HashMap;

const EXCLUDE_LIST: &[char] = &[',', ':', '!', '&', '@', '$', '%', '^', '.'];

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut map = HashMap::new();

    for temp_word in words.split_whitespace().into_iter() {
        for word in temp_word.split(&[',', '\n', '\t']).collect::<Vec<&str>>() {
            let mut word = word.replace(&EXCLUDE_LIST[..], "");
            if word != "" {
                let chars = word.chars().collect::<Vec<char>>();
                if chars[0] == '\'' && chars[chars.len() - 1] == '\'' {
                    word = word.replace('\'', "");
                }
            }
            if word != "" {
                map.entry(word.to_lowercase().to_string())
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
        }
    }

    map
}

pub fn word_count_1(words: &str) -> HashMap<String, u32> {
    words
        .split(|c: char| !c.is_alphanumeric() && c != '\'')
        .filter(|w| !w.is_empty())
        .map(|w| w.trim_matches('\'').to_lowercase())
        .fold(HashMap::new(), |mut acc, w| {
            *acc.entry(w).or_default() += 1;
            acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_word_count(s: &str, pairs: &[(&str, u32)]) {
        // The reason for the awkward code in here is to ensure that the failure
        // message for assert_eq! is as informative as possible. A simpler
        // solution would simply check the length of the map, and then
        // check for the presence and value of each key in the given pairs vector.
        let mut m: HashMap<String, u32> = crate::word_count::word_count(s);
        for &(k, v) in pairs.iter() {
            assert_eq!((k, m.remove(&k.to_string()).unwrap_or(0)), (k, v));
        }
        // may fail with a message that clearly shows all extra pairs in the map
        assert_eq!(m.iter().collect::<Vec<(&String, &u32)>>(), vec![]);
    }

    #[test]
    fn test_count_one_word() {
        check_word_count("word", &[("word", 1)]);
    }

    #[test]
    fn test_count_one_of_each() {
        check_word_count("one of each", &[("one", 1), ("of", 1), ("each", 1)]);
    }

    #[test]
    fn test_count_multiple_occurrences() {
        check_word_count(
            "one fish two fish red fish blue fish",
            &[("one", 1), ("fish", 4), ("two", 1), ("red", 1), ("blue", 1)],
        );
    }

    #[test]
    fn cramped_lists() {
        check_word_count("one,two,three", &[("one", 1), ("two", 1), ("three", 1)]);
    }

    #[test]
    fn expanded_lists() {
        check_word_count("one\ntwo\nthree", &[("one", 1), ("two", 1), ("three", 1)]);
    }

    #[test]
    fn test_ignore_punctuation() {
        check_word_count(
            "car : carpet as java : javascript!!&@$%^&",
            &[
                ("car", 1),
                ("carpet", 1),
                ("as", 1),
                ("java", 1),
                ("javascript", 1),
            ],
        );
    }

    #[test]
    fn test_include_numbers() {
        check_word_count(
            "testing, 1, 2 testing",
            &[("testing", 2), ("1", 1), ("2", 1)],
        );
    }
    #[test]
    fn test_normalize_case() {
        check_word_count("go Go GO Stop stop", &[("go", 3), ("stop", 2)]);
    }
    #[test]
    fn with_apostrophes() {
        check_word_count(
            "First: don't laugh. Then: don't cry.",
            &[
                ("first", 1),
                ("don't", 2),
                ("laugh", 1),
                ("then", 1),
                ("cry", 1),
            ],
        );
    }
    #[test]
    fn with_quotations() {
        check_word_count(
            "Joe can't tell between 'large' and large.",
            &[
                ("joe", 1),
                ("can't", 1),
                ("tell", 1),
                ("between", 1),
                ("large", 2),
                ("and", 1),
            ],
        );
    }
    #[test]
    fn multiple_spaces_not_detected_as_a_word() {
        check_word_count(
            " multiple   whitespaces",
            &[("multiple", 1), ("whitespaces", 1)],
        );
    }
}