enum ReferenceLinkedList<'a, T> {
    Cons(T, &'a ReferenceLinkedList<'a, T>),
    Nil,
}

impl<'a, T> ReferenceLinkedList<'a, T>
where
    T: PartialEq,
{
    fn contains(self: &Self, x: &T) -> bool {
        match self {
            ReferenceLinkedList::Cons(head, tail) => {
                if head == x {
                    return true;
                } else {
                    Self::contains(tail, x)
                }
            }
            ReferenceLinkedList::Nil => false,
        }
    }
}

struct Solution;

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        for (i, row) in board.iter().enumerate() {
            for (j, _) in row.iter().enumerate() {
                if Solution::inner_exist(&board, &word, (i, j), &ReferenceLinkedList::Nil) {
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
        used: &ReferenceLinkedList<(usize, usize)>,
    ) -> bool {
        if word.is_empty() {
            return true;
        }

        let mut word = word.chars();

        let current_first_letter = word.next().unwrap();

        if board[from_x][from_y] != current_first_letter {
            return false;
        }

        let used = ReferenceLinkedList::Cons(from, used);

        [(-1, 0), (0, 1), (1, 0), (0, -1)]
            .map(|(delta_x, delta_y)| (from_x as i32 + delta_x, from_y as i32 + delta_y))
            .into_iter()
            .filter(|(x, y)| 0 <= *x && 0 <= *y)
            .map(|(x, y)| (x as usize, y as usize))
            .filter(|(x, y)| *x < board.len() && *y < board[*x].len())
            .filter(|new_from| !used.contains(&new_from.clone()))
            .any(|new_from| Solution::inner_exist(board, word.as_str(), new_from.clone(), &used))
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
