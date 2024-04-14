impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s: String = s
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect();

        let mut front = s.chars();
        let mut back = s.chars().rev();
        for _ in 0..(s.len() / 2) {
            let back = back.next();
            let front = front.next();

            if front != back {
                return false;
            }
        }

        true
    }
}

#[test]
fn case2() {
    assert_eq!(
        true,
        Solution::is_palindrome("A man, a plan, a canal: Panama".to_string())
    )
}

#[test]
fn case3() {
    assert_eq!(false, Solution::is_palindrome("race a car".to_string()))
}

struct Solution;

fn main() {}
