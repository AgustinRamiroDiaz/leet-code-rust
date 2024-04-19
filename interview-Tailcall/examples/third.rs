/*
 * Complete the 'countBetween' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY arr
 *  2. INTEGER_ARRAY low
 *  3. INTEGER_ARRAY high
 */

fn countBetween(arr: &[i32], low: &[i32], high: &[i32]) -> Vec<i32> {
    let out_length = low.len();
    let mut out = Vec::new();

    for index in 0..out_length {
        let mut sum = 0;
        for element in arr {
            let l = low[index];
            let h = high[index];

            if &l <= element && element <= &h {
                sum += 1;
            }
        }
        out.push(sum);
    }
    out
}
#[test]
fn case1() {
    let arr = [1, 2, 2, 3, 4];
    let low = [0, 2];
    let high = [2, 4];
    let expected = vec![3, 4];
    assert_eq!(expected, countBetween(&arr, &low, &high))
}

#[test]
fn case2() {
    let arr = [4, 8, 7];
    let low = [2, 4];
    let high = [8, 4];
    let expected = vec![3, 1];
    assert_eq!(expected, countBetween(&arr, &low, &high))
}

fn main() {}
