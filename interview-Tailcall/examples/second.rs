/*
 * Complete the 'findLargestSquareSize' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts 2D_INTEGER_ARRAY samples as parameter.
 */

fn findLargestSquareSize(samples: &[Vec<i32>]) -> i32 {
    /*

    # Performance

    This runs in O(n^2)


    # Train of thought

    we could create another matrix of the same size and fill it with the distance to the left and up of the rectangles of defectuous products

    then, we should search for the maximum values which can form a square

    Second thought: could we directly go with the size of the square by taking into account left, diagonal and up positions?

    */

    if samples.len() == 0 || samples[0].len() == 0 {
        return 0;
    }

    // distances in order (left, up)
    let mut squares: Vec<Vec<i32>> = samples
        .iter()
        .map(|row| Vec::with_capacity(row.len()))
        .collect();

    // Top row
    for i in &samples[0] {
        squares[0].push(*i);
    }

    // Left column
    for row in 1..samples.len() {
        squares[row].push(samples[row][0])
    }

    for row in 1..samples.len() {
        for column in 1..samples[0].len() {
            let element = samples[row][column];

            if element == 0 {
                squares[row].push(0);
            } else {
                // it's a 1
                let left = squares[row][column - 1];
                let diagonal = squares[row - 1][column - 1];
                let top = squares[row - 1][column];
                let value = left.min(diagonal).min(top);
                squares[row].push(value + 1)
            }
        }
    }

    let mut max = 0;

    for row in squares {
        for element in row {
            max = max.max(element);
        }
    }

    max
}

// #[test]
// fn case1() {
//     let input = ["3", "2", "1", "2", "3"].map(|s| s.to_string());
//     let expected = 4;
//     assert_eq!(expected, findLargestSquareSize(&input, 2))
// }

#[test]
fn case2() {
    let mut x = Vec::with_capacity(10);

    x[0] = 1;

    println!("{x:?}")
}

fn main() {}
