use std::collections::HashMap;

pub enum Category {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Choice,
    Yacht,
}

type Dice = [u8; 5];

pub fn score(dice: Dice, category: Category) -> u8 {
    match category {
        Category::Ones => dice.to_vec().iter().filter(|num| **num == 1).sum(),
        Category::Twos => dice.to_vec().iter().filter(|num| **num == 2).sum(),
        Category::Threes => dice.to_vec().iter().filter(|num| **num == 3).sum(),
        Category::Fours => dice.to_vec().iter().filter(|num| **num == 4).sum(),
        Category::Fives => dice.to_vec().iter().filter(|num| **num == 5).sum(),
        Category::Sixes => dice.to_vec().iter().filter(|num| **num == 6).sum(),
        Category::FullHouse => {
            let mut map = HashMap::new();

            dice.iter().for_each(|num| {
                map.entry(num)
                    .and_modify(|counter| *counter += 1)
                    .or_insert(1);
            });

            let mut sum = 0;
            let mut total = 0;
            map.iter().for_each(|(key, value)| {
                if *value == 2 || *value == 3 {
                    sum += *value * *key;
                    total += *value;
                }
            });

            if total == 5 {
                sum
            } else {
                0
            }
        }
        Category::FourOfAKind => {
            let mut map = HashMap::new();

            dice.iter().for_each(|num| {
                map.entry(num)
                    .and_modify(|counter| *counter += 1)
                    .or_insert(1);
            });

            let mut sum = 0;
            map.iter().for_each(|(key, value)| {
                if *value == 4 || *value == 5 {
                    sum += 4 * *key;
                }
            });

            sum
        }
        Category::LittleStraight => {
            let mut dice: Vec<u8> = dice.clone().to_vec();
            dice.sort();
            let mut count = 1;

            for num in dice.iter() {
                if *num != count {
                    return 0;
                }
                count += 1;
            }
            30
        }
        Category::BigStraight => {
            let mut dice: Vec<u8> = dice.clone().to_vec();
            dice.sort();
            let mut count = 2;

            for num in dice.iter() {
                if *num != count {
                    return 0;
                }
                count += 1;
            }
            30
        }
        Category::Choice => dice.to_vec().iter().sum(),
        Category::Yacht => {
            let target = dice[0];

            for num in dice.iter() {
                if *num != target {
                    return 0;
                }
            }

            50
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::yacht::*;
    #[test]
    fn test_yacht() {
        let expected = 50;
        assert_eq!(score([5, 5, 5, 5, 5], Category::Yacht), expected);
    }
    #[test]
    fn test_not_yacht() {
        let expected = 0;
        assert_eq!(score([1, 3, 3, 2, 5], Category::Yacht), expected);
    }
    #[test]
    fn test_ones() {
        let expected = 3;
        assert_eq!(score([1, 1, 1, 3, 5], Category::Ones), expected);
    }
    #[test]
    fn test_ones_out_of_order() {
        let expected = 3;
        assert_eq!(score([3, 1, 1, 5, 1], Category::Ones), expected);
    }
    #[test]
    fn test_no_ones() {
        let expected = 0;
        assert_eq!(score([4, 3, 6, 5, 5], Category::Ones), expected);
    }
    #[test]
    fn test_twos() {
        let expected = 2;
        assert_eq!(score([2, 3, 4, 5, 6], Category::Twos), expected);
    }
    #[test]
    fn test_fours() {
        let expected = 8;
        assert_eq!(score([1, 4, 1, 4, 1], Category::Fours), expected);
    }
    #[test]
    fn test_yacht_counted_as_threes() {
        let expected = 15;
        assert_eq!(score([3, 3, 3, 3, 3], Category::Threes), expected);
    }
    #[test]
    fn test_yacht_of_3s_counted_as_fives() {
        let expected = 0;
        assert_eq!(score([3, 3, 3, 3, 3], Category::Fives), expected);
    }
    #[test]
    fn test_fives() {
        let expected = 10;
        assert_eq!(score([1, 5, 3, 5, 3], Category::Fives), expected);
    }
    #[test]
    fn test_sixes() {
        let expected = 6;
        assert_eq!(score([2, 3, 4, 5, 6], Category::Sixes), expected);
    }
    #[test]
    fn test_full_house_two_small_three_big() {
        let expected = 16;
        assert_eq!(score([2, 2, 4, 4, 4], Category::FullHouse), expected);
    }
    #[test]
    fn test_full_house_three_small_two_big() {
        let expected = 19;
        assert_eq!(score([5, 3, 3, 5, 3], Category::FullHouse), expected);
    }
    #[test]
    fn test_two_pair_is_not_a_full_house() {
        let expected = 0;
        assert_eq!(score([2, 2, 4, 4, 5], Category::FullHouse), expected);
    }
    #[test]
    fn test_four_of_a_kind_is_not_a_full_house() {
        let expected = 0;
        assert_eq!(score([1, 4, 4, 4, 4], Category::FullHouse), expected);
    }
    #[test]
    fn test_yacht_is_not_a_full_house() {
        let expected = 0;
        assert_eq!(score([2, 2, 2, 2, 2], Category::FullHouse), expected);
    }
    #[test]
    fn test_four_of_a_kind() {
        let expected = 24;
        assert_eq!(score([6, 6, 4, 6, 6], Category::FourOfAKind), expected);
    }
    #[test]
    fn test_yacht_can_be_scored_as_four_of_a_kind() {
        let expected = 12;
        assert_eq!(score([3, 3, 3, 3, 3], Category::FourOfAKind), expected);
    }
    #[test]
    fn test_full_house_is_not_four_of_a_kind() {
        let expected = 0;
        assert_eq!(score([3, 3, 3, 5, 5], Category::FourOfAKind), expected);
    }
    #[test]
    fn test_little_straight() {
        let expected = 30;
        assert_eq!(score([3, 5, 4, 1, 2], Category::LittleStraight), expected);
    }
    #[test]
    fn test_little_straight_as_big_straight() {
        let expected = 0;
        assert_eq!(score([1, 2, 3, 4, 5], Category::BigStraight), expected);
    }
    #[test]
    fn test_four_in_order_but_not_a_little_straight() {
        let expected = 0;
        assert_eq!(score([1, 1, 2, 3, 4], Category::LittleStraight), expected);
    }
    #[test]
    fn test_no_pairs_but_not_a_little_straight() {
        let expected = 0;
        assert_eq!(score([1, 2, 3, 4, 6], Category::LittleStraight), expected);
    }
    #[test]
    fn test_minimum_is_1_maximum_is_5_but_not_a_little_straight() {
        let expected = 0;
        assert_eq!(score([1, 1, 3, 4, 5], Category::LittleStraight), expected);
    }
    #[test]
    fn test_big_straight() {
        let expected = 30;
        assert_eq!(score([4, 6, 2, 5, 3], Category::BigStraight), expected);
    }
    #[test]
    fn test_big_straight_as_little_straight() {
        let expected = 0;
        assert_eq!(score([6, 5, 4, 3, 2], Category::LittleStraight), expected);
    }
    #[test]
    fn test_no_pairs_but_not_a_big_straight() {
        let expected = 0;
        assert_eq!(score([6, 5, 4, 3, 1], Category::BigStraight), expected);
    }
    #[test]
    fn test_choice() {
        let expected = 23;
        assert_eq!(score([3, 3, 5, 6, 6], Category::Choice), expected);
    }
    #[test]
    fn test_yacht_as_choice() {
        let expected = 10;
        assert_eq!(score([2, 2, 2, 2, 2], Category::Choice), expected);
    }
}
