fn arrayManipulation(n: i32, queries: &[Vec<i32>]) -> i64 {
    let mut array = vec![];

    (0..n).for_each(|_| array.push(0));

    for query in queries {
        let from = query[0] - 1;
        let to = query[1] - 1;
        let value = query[2];

        for index in from..=to {
            array[index as usize] += value;
        }
    }

    array.iter().fold(0, |a, b| a.max(*b)) as i64
}

#[test]
fn case1() {
    let queries = [vec![1, 5, 3], vec![4, 8, 7], vec![6, 9, 1]];

    assert_eq!(10, arrayManipulation(10, &queries))
}

#[test]
fn case2() {
    let queries = [vec![1, 2, 100], vec![2, 5, 100], vec![3, 4, 100]];

    assert_eq!(200, arrayManipulation(5, &queries))
}

fn main() {}
