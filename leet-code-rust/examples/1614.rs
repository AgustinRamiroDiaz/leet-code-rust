impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut max_depth = 0;
        let mut current_depth = 0;
        for character in s.chars() {
            if character == '(' {
                current_depth += 1;
                max_depth = max_depth.max(current_depth);
            } else if character == ')' {
                current_depth -= 1;
            }
        }

        max_depth
    }
}

#[test]
fn case1() {
    assert_eq!(Solution::max_depth("(1+(2*3)+((8)/4))+1".to_string()), 3);
}

#[test]
fn case2() {
    assert_eq!(Solution::max_depth("(1)+((2))+(((3)))".to_string()), 3);
}

struct Solution;

fn main() {}
