impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();

        for character in s.chars() {
            match character {
                '(' => stack.push(character),
                ')' => {
                    if !stack.pop().is_some_and(|c| c == '(') {
                        return false;
                    }
                }
                '{' => stack.push(character),
                '}' => {
                    if !stack.pop().is_some_and(|c| c == '{') {
                        return false;
                    }
                }
                '[' => stack.push(character),
                ']' => {
                    if !stack.pop().is_some_and(|c| c == '[') {
                        return false;
                    }
                }
                _ => unreachable!(),
            }
        }

        stack.is_empty()
    }
}

#[test]
fn case1() {
    assert_eq!(true, Solution::is_valid("()".to_string()))
}
#[test]
fn case2() {
    assert_eq!(true, Solution::is_valid("()[]{}".to_string()))
}
#[test]
fn case3() {
    assert_eq!(false, Solution::is_valid("(]".to_string()))
}
#[test]
fn case4() {
    assert_eq!(true, Solution::is_valid("{[({[{{}}]})]}".to_string()))
}
#[test]
fn case5() {
    assert_eq!(false, Solution::is_valid("{[({[{{}}])]}".to_string()))
}

struct Solution;

fn main() {}
