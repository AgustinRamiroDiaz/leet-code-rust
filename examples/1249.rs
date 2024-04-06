impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut closed_vs_open_counter = 0;
        let mut reverse_result = String::new();
        for character in s.chars().rev() {
            if character == '(' {
                closed_vs_open_counter -= 1;
            } else if character == ')' {
                closed_vs_open_counter += 1;
            }

            if closed_vs_open_counter == -1 {
                closed_vs_open_counter += 1
            } else {
                reverse_result.push(character);
            }
        }

        let mut open_vs_closed_counter = 0;
        let mut result = String::new();
        for character in reverse_result.chars().rev() {
            if character == '(' {
                open_vs_closed_counter += 1;
            } else if character == ')' {
                open_vs_closed_counter -= 1;
            }

            if open_vs_closed_counter == -1 {
                open_vs_closed_counter += 1
            } else {
                result.push(character);
            }
        }

        result
    }
}

struct Solution;

#[test]
fn case1() {
    assert_eq!(
        Solution::min_remove_to_make_valid("lee(t(c)o)de)".to_string()),
        "lee(t(c)o)de".to_string()
    );
}

#[test]
fn case2() {
    assert_eq!(
        Solution::min_remove_to_make_valid("a)b(c)d".to_string()),
        "ab(c)d".to_string()
    );
}

#[test]
fn case3() {
    assert_eq!(
        Solution::min_remove_to_make_valid("))((".to_string()),
        "".to_string()
    );
}

#[test]
fn case4() {
    assert_eq!(
        Solution::min_remove_to_make_valid("))((((((())))))((((((((((((((((((((())".to_string()),
        "(((((())))))(())".to_string()
    );
}

fn main() {}
