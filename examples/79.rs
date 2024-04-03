use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        for (i, row) in board.iter().enumerate() {
            for (j, _) in row.iter().enumerate() {
                if Solution::inner_exist(&board, &word, (i, j), &mut HashSet::new()) {
                    return true;
                }
            }
        }

        false
    }

    fn inner_exist(
        board: &Vec<Vec<char>>,
        word: &str,
        from @ (from_x, from_y): (usize, usize),
        used: &mut HashSet<(usize, usize)>,
    ) -> bool {
        if word.is_empty() {
            return true;
        }

        let mut word = word.chars();

        let current_first_letter = word.next().unwrap();

        if board[from_x][from_y] != current_first_letter {
            return false;
        }

        used.insert(from);

        [(-1, 0), (0, 1), (1, 0), (0, -1)]
            .map(|(delta_x, delta_y)| (from_x as i32 + delta_x, from_y as i32 + delta_y))
            .into_iter()
            .filter(|(x, y)| {
                0 <= *x && *x < board.len() as i32 && 0 < *y && *y < board[*x as usize].len() as i32
            })
            .map(|(x, y)| (x as usize, y as usize))
            .filter(|new_from| !used.contains(&new_from.clone()))
            .any(|new_from| {
                Solution::inner_exist(board, word.as_str(), new_from.clone(), &mut used.clone())
            })
    }
}

#[test]
fn my() {
    let x = "hola";

    let mut iter = x.chars();

    iter.next();

    assert_eq!(iter.as_str(), "ola")
}

#[test]
fn case1() {
    assert_eq!(
        Solution::exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E']
            ],
            "ABCCED".to_string()
        ),
        true
    );
}

#[test]
fn case2() {
    assert_eq!(
        Solution::exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E']
            ],
            "SEE".to_string()
        ),
        true
    );
}

#[test]
fn case3() {
    assert_eq!(
        Solution::exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E']
            ],
            "ABCB".to_string()
        ),
        false
    );
}

fn main() {}
