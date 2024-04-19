use std::{collections::HashMap, iter::repeat};

impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut distance_to_zero: Vec<Vec<Option<usize>>> = (0..mat.len())
            .map(|_| repeat(None).take(mat[0].len()).collect())
            .collect();

        for (row_index, row) in mat.iter().enumerate() {
            for (col_index, _) in row.iter().enumerate() {
                Self::search(&mat, &mut distance_to_zero, row_index, col_index);
            }
        }

        distance_to_zero
            .iter()
            .map(|row| row.iter().map(|x| x.unwrap() as i32).collect())
            .collect()
    }

    fn search(
        mat: &Vec<Vec<i32>>,
        distance_to_zero: &mut Vec<Vec<Option<usize>>>,
        from_row: usize,
        from_col: usize,
    ) -> usize {
        if let Some(x) = distance_to_zero[from_row][from_col] {
            return x;
        }
        let out = if mat[from_row][from_col] == 0 {
            0 as usize
        } else {
            let neighbours = [(1, 0), (0, 1), (0, -1), (-1, 0)]
                .iter()
                .map(|(x, y)| ((from_row as i32 + x, from_col as i32 + y)))
                .filter(|(x, y)| {
                    mat.get(*x as usize)
                        .is_some_and(|row| row.get(*y as usize).is_some())
                })
                .map(|(x, y)| (x as usize, y as usize));

            neighbours
                .map(|(x, y)| Self::search(mat, distance_to_zero, x, y))
                .min()
                .unwrap()
                + 1
        };
        distance_to_zero[from_row][from_col] = Some(out);
        out
    }
}

#[test]
fn case1() {
    assert_eq!(
        vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]],
        Solution::update_matrix(vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 2, 1]])
    );
}

#[test]
fn case2() {
    assert_eq!(
        vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]],
        Solution::update_matrix(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]])
    );
}

struct Solution;

fn main() {}
