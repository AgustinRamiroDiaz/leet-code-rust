use std::collections::VecDeque;

impl Solution {
    pub fn deck_revealed_increasing(deck: Vec<i32>) -> Vec<i32> {
        let mut deck = deck;
        deck.sort();
        deck.reverse();

        let mut result = VecDeque::new();

        for card in deck {
            if result.len() > 0 {
                result.rotate_right(1);
            }
            result.push_front(card);
        }

        result.into()
    }
}

#[test]
fn case0() {
    assert_eq!(
        vec![2, 13, 3, 11, 5, 17, 7],
        Solution::deck_revealed_increasing(vec![17, 13, 11, 2, 3, 5, 7])
    )
}

struct Solution;

fn main() {}
