use std::collections::{HashMap, VecDeque};

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

#[derive(Debug)]
enum Edge {
    Token(Token),
    Empty,
}

type NodeId = usize;

#[derive(Debug)]
struct NonDeterministicStateMachine {
    start_node: NodeId,
    end_node: NodeId,
    edges: HashMap<NodeId, Vec<(Edge, NodeId)>>,
}

fn meta_tokens_to_ndstm(mut pattern: VecDeque<MetaToken>) -> NonDeterministicStateMachine {
    let start_node = 0;

    let mut node_id = start_node;

    let mut edges = HashMap::new();
    for token in pattern {
        let new_node_id = node_id + 1;
        let entry = edges.entry(node_id).or_insert(vec![]);
        match token {
            MetaToken::Klein(token) => {
                entry.push((Edge::Empty, new_node_id));
                entry.push((Edge::Token(token), new_node_id));
            }
            MetaToken::Single(token) => {
                entry.push((Edge::Token(token), new_node_id));
            }
        }
        node_id = new_node_id;
    }

    let end_node = node_id;

    NonDeterministicStateMachine {
        start_node,
        end_node,
        edges,
    }
}

fn step(ndsm: &NonDeterministicStateMachine, from: NodeId, character: char) -> Vec<NodeId> {
    let edges = ndsm.edges.get(&from).unwrap(); // we don't expect this to be called with wrong from

    edges
        .iter()
        .flat_map(|(edge, to)| match edge {
            Edge::Empty => step(ndsm, *to, character),
            Edge::Token(Token::Dot) => vec![*to],
            Edge::Token(Token::Character(edge_character)) => {
                if character == *edge_character {
                    vec![*to]
                } else {
                    vec![]
                }
            }
        })
        .collect()
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

        let ndsm = meta_tokens_to_ndstm(tokens);
        println!("{:?}", ndsm);
        let mut node_pointers = vec![ndsm.start_node];

        for character in s.chars() {
            if node_pointers.is_empty() {
                return false;
            }
            node_pointers = node_pointers
                .iter()
                .flat_map(|node| step(&ndsm, *node, character))
                .collect();
        }

        node_pointers.contains(&ndsm.end_node)
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
// - parse pattern and convert into non-deterministic state machine
//   - this can be modeled with a Directional Cyclic Graph. The graph could be a HashMap<NodeId, (Token Edge, NodeId)>
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
