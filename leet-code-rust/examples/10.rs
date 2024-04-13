use std::collections::VecDeque;

#[derive(Clone, Debug)]
enum Token {
    Character(char),
    Dot,
}

#[derive(Clone, Debug)]
enum MetaToken {
    Single(Token),
    Klein(Token),
}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut tokens = VecDeque::new();

        for character in p.chars() {
            let meta_token = match character {
                '*' => {
                    if let Some(MetaToken::Single(last)) = tokens.pop_back() {
                        MetaToken::Klein(last)
                    } else {
                        // we assume the pattern is well formed
                        panic!()
                    }
                }
                '.' => MetaToken::Single(Token::Dot),
                character => MetaToken::Single(Token::Character(character)),
            };

            tokens.push_back(meta_token);
        }

        Self::is_match_inner(s, tokens)
    }

    fn is_match_inner(s: String, mut pattern: VecDeque<MetaToken>) -> bool {
        println!("comparing {:?} and {:?}", s, pattern);
        if let Some(meta_token) = pattern.pop_front() {
            match meta_token {
                MetaToken::Klein(token) => {
                    Self::is_match_inner(s.clone(), pattern.clone()) || {
                        if s.is_empty() {
                            return false;
                        }
                        let mut s = s.chars();

                        let next_s = s.next().unwrap();
                        match token {
                            Token::Character(character) => {
                                if character != next_s {
                                    return false;
                                }
                            }
                            Token::Dot => {}
                        };

                        let s = s.as_str().to_string();
                        let mut pattern2 = pattern.clone();
                        pattern2.push_front(MetaToken::Klein(token));
                        Self::is_match_inner(s.clone(), pattern2)
                            || Self::is_match_inner(s, pattern)
                    }
                }
                MetaToken::Single(Token::Character(character)) => {
                    if s.is_empty() {
                        return false;
                    }
                    let mut s = s.chars();
                    if s.next().unwrap() != character {
                        return false;
                    }
                    Self::is_match_inner(s.as_str().to_string(), pattern)
                }
                MetaToken::Single(Token::Dot) => {
                    if s.is_empty() {
                        return false;
                    }
                    let mut s = s.chars();
                    s.next().unwrap();
                    Self::is_match_inner(s.as_str().to_string(), pattern)
                }
            }
        } else {
            s.is_empty()
        }
    }
}

#[test]
fn case1() {
    assert_eq!(
        true,
        Solution::is_match(
            "una abeja se baña".to_string(),
            "una.*ja.s*e...ñ.".to_string()
        )
    )
}
#[test]
fn case2() {
    assert_eq!(
        false,
        Solution::is_match(
            "una abeja se baña".to_string(),
            "una.*ja.s*e...ñ".to_string()
        )
    )
}

#[test]
fn case3() {
    assert_eq!(
        true,
        Solution::is_match("aab".to_string(), "c*a*b".to_string())
    )
}

#[test]
fn case4() {
    assert_eq!(
        false,
        Solution::is_match("mississippi".to_string(), "mis*is*p*.".to_string())
    )
}

// TODO: this doesn't work due to algoritmic complexity
// I think the solution would be to use state machines
// The pseudocode could be:
// - parse pattern and convert into non-deterministic state machine (with a DAG)
// - create a set of pointers, initialized to [start node]
// - on each iteration of the string input, step all node pointers to all their possible new states
//   - if they can go to multiple places, get all of them
//   - if they can't go anywhere, don't add nothing
// - base case is when either the set is empty, which means that there's no solution, or the last node is present in the set, which means there's a solution
#[test]
fn case5() {
    assert_eq!(
        false,
        Solution::is_match(
            "aaaaaaaaaaaaab".to_string(),
            "a*a*a*a*a*a*a*a*a*c".to_string()
        )
    )
}

struct Solution;

fn main() {}