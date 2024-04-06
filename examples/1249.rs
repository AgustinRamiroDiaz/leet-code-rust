impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let open_removed = Self::remove_closing(s.chars().rev(), ')', '(');

        Self::remove_closing(open_removed.chars().rev(), '(', ')')
    }

    fn remove_closing(
        s: impl Iterator<Item = char>,
        open_character: char,
        closing_character: char,
    ) -> String {
        let mut open_vs_closed_counter = 0;

        s.filter(|character| {
            if character == &open_character {
                open_vs_closed_counter += 1;
            } else if character == &closing_character {
                open_vs_closed_counter -= 1;
            }

            if open_vs_closed_counter == -1 {
                open_vs_closed_counter += 1;
                false
            } else {
                true
            }
        })
        .collect()
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
