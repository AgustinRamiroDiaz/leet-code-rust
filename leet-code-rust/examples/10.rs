use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
};

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

impl Display for NonDeterministicStateMachine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n", self.start_node)?;

        let mut edges: Vec<_> = self.edges.iter().collect();

        edges.sort_by(|&a, &b| a.0.cmp(b.0));

        for (from, edges) in edges {
            for (edge, to) in edges {
                write!(f, "{from}-{edge:?}->{to}\n")?;
            }
        }

        write!(f, "{}", self.end_node)?;
        Ok(())
    }
}

fn meta_tokens_to_ndstm(pattern: VecDeque<MetaToken>) -> NonDeterministicStateMachine {
    let start_node = 0;

    let mut node_id = start_node;

    let mut edges = HashMap::new();
    for token in pattern {
        let new_node_id = node_id + 1;
        let entry = edges.entry(node_id).or_insert(vec![]);
        match token {
            MetaToken::Klein(token) => {
                entry.push((Edge::Empty, new_node_id));
                edges
                    .entry(new_node_id)
                    .or_insert(vec![])
                    .push((Edge::Token(token), new_node_id));
            }
            MetaToken::Single(token) => {
                entry.push((Edge::Token(token), new_node_id));
            }
        }
        node_id = new_node_id;
    }

    let end_node = node_id;

    edges.entry(end_node).or_insert(vec![]);

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

fn traverse_empty(ndsm: &NonDeterministicStateMachine, from: NodeId) -> Vec<NodeId> {
    let edges = ndsm.edges.get(&from).unwrap(); // we don't expect this to be called with wrong from

    let mut out: Vec<_> = edges
        .iter()
        .flat_map(|(edge, to)| match edge {
            Edge::Empty => traverse_empty(ndsm, *to),
            Edge::Token(_) => {
                vec![]
            }
        })
        .collect();

    out.push(from);

    out
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
        println!("{}", ndsm);
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

        node_pointers = node_pointers
            .iter()
            .flat_map(|&node| traverse_empty(&ndsm, node))
            .collect();

        node_pointers.contains(&ndsm.end_node)
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

#[test]
fn case6() {
    assert_eq!(true, Solution::is_match("a".to_string(), "ab*".to_string()))
}

struct Solution;

fn main() {}
