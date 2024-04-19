use std::collections::{BinaryHeap, HashMap, HashSet};

fn getMinimumSize(requests: &[String], k: i32) -> i32 {
    if requests.len() as i32 - 1 < k {
        return -1;
    }

    let mut top = requests.len() - 1;
    let mut bottom = 1;

    while top != bottom {
        let middle = (top + bottom) / 2;
        if cache_hits(requests, middle) > k {
            top = middle;
        } else {
            bottom = middle
        }
    }

    if cache_hits(requests, bottom) > k {
        return bottom as i32;
    }

    -1
}

fn cache_hits(requests: &[String], size: usize) -> i32 {
    let mut cache = Vec::new();

    let mut hits = 0;
    for element in requests {
        if cache.contains(&element) {
            hits += 1;
        }
        cache.retain(|&x| x != element);
        cache.push(element);

        while cache.len() > size {
            cache.pop();
        }
    }

    hits
}

#[test]
fn case1() {
    let input = ["3", "2", "1", "2", "3"].map(|s| s.to_string());
    let expected = 4;
    assert_eq!(expected, getMinimumSize(&input, 2))
}

#[test]
fn case2() {
    let input = ["3", "2", "1", "2", "3"].map(|s| s.to_string());
    let expected = 2;
    assert_eq!(expected, getMinimumSize(&input, 1))
}

#[test]
fn case3() {
    let input = [
        "item3", "item4", "item2", "item6", "item4", "item3", "item7", "item4", "item3", "item6",
        "item3", "item4", "item8", "item4", "item6",
    ]
    .map(|s| s.to_string());
    let expected = 3;
    assert_eq!(expected, getMinimumSize(&input, 6))
}

fn main() {
    println!("Hello, world!");
}
