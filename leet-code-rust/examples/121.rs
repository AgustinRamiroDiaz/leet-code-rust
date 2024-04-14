impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min = None;

        let min_to_the_left = prices.iter().map(|&x| {
            match min {
                Some(value) => {
                    if x < value {
                        min = Some(x);
                    }
                }
                None => min = Some(x),
            }
            min.unwrap()
        });

        let benefits = prices
            .iter()
            .zip(min_to_the_left)
            .map(|(price, min_to_the_left)| price - min_to_the_left);

        benefits.max().unwrap_or(0)
    }
}

#[test]
fn case2() {
    assert_eq!(5, Solution::max_profit(vec![7, 1, 5, 3, 6, 4]))
}

#[test]
fn case3() {
    assert_eq!(0, Solution::max_profit(vec![7, 6, 4, 3, 1]))
}

struct Solution;

fn main() {}
