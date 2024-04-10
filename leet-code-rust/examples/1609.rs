// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;

fn strictly_increasing<T>(iterator: impl Iterator<Item = T>) -> bool
where
    T: PartialOrd + Copy,
{
    let mut current = None;

    for i in iterator {
        match current {
            Some(x) => {
                if x >= i {
                    return false;
                }
                current = Some(i);
            }
            None => {
                current = Some(i);
            }
        }
    }

    true
}

impl Solution {
    pub fn is_even_odd_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut current_row: Vec<_>;
        let mut row_index = 0;

        current_row = vec![root].into_iter().flatten().collect();
        while !current_row.is_empty() {
            let current_values = current_row.iter().map(|n| n.as_ref().borrow().val);

            if row_index % 2 == 0 {
                if !current_values.clone().all(|v| v % 2 != 0) {
                    return false;
                }
                if !strictly_increasing(current_values) {
                    return false;
                }
            } else {
                if !current_values.clone().all(|v| v % 2 == 0) {
                    return false;
                }
                if !strictly_increasing(current_values.rev()) {
                    return false;
                }
            }

            row_index += 1;
            current_row = current_row
                .into_iter()
                .flat_map(|n| {
                    [
                        n.as_ref().borrow().left.clone(),
                        n.as_ref().borrow().right.clone(),
                    ]
                    .into_iter()
                    .flatten()
                })
                .collect();
        }

        true
    }
}

fn from(input: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    use std::collections::VecDeque;

    let mut iter = input.into_iter();
    let root = Rc::new(RefCell::new(TreeNode::new(iter.next().unwrap().unwrap())));
    let mut queue = VecDeque::new();
    queue.push_back(Rc::clone(&root));

    while let Some(node) = queue.pop_front() {
        if let Some(Some(val)) = iter.next() {
            let left = Rc::new(RefCell::new(TreeNode::new(val)));
            node.borrow_mut().left = Some(Rc::clone(&left));
            queue.push_back(Rc::clone(&left));
        }

        if let Some(Some(val)) = iter.next() {
            let right = Rc::new(RefCell::new(TreeNode::new(val)));
            node.borrow_mut().right = Some(Rc::clone(&right));
            queue.push_back(Rc::clone(&right));
        }
    }

    Some(root)
}

#[test]
fn case1() {
    assert_eq!(
        Solution::is_even_odd_tree(Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 10,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                right: None,
            }))),
        })))),
        true
    );
}

#[test]
fn case2() {
    // [13,34,32,23,25,27,29,44,40,36,34,30,30,28,26,3,7,9,11,15,17,21,25,null,null,27,31,35,null,37,null,30,null,26,null,null,null,24,null,20,16,12,10,null,null,8,null,null,null,null,null,6,null,null,null,null,null,15,19,null,null,null,null,23,null,27,29,33,37,null,null,null,null,null,null,48,null,null,null,46,null,null,null,42,38,34,32,null,null,null,null,19]
    let example = from(vec![
        Some(13),
        Some(34),
        Some(32),
        Some(23),
        Some(25),
        Some(27),
        Some(29),
        Some(44),
        Some(40),
        Some(36),
        Some(34),
        Some(30),
        Some(30),
        Some(28),
        Some(26),
        Some(3),
        Some(7),
        Some(9),
        Some(11),
        Some(15),
        Some(17),
        Some(21),
        Some(25),
        None,
        None,
        Some(27),
        Some(31),
        Some(35),
        None,
        Some(37),
        None,
        Some(30),
        None,
        Some(26),
        None,
        None,
        None,
        Some(24),
        None,
        Some(20),
        Some(16),
        Some(12),
        Some(10),
        None,
        None,
        Some(8),
        None,
        None,
        None,
        None,
        None,
        Some(6),
        None,
        None,
        None,
        None,
        None,
        Some(15),
        Some(19),
        None,
        None,
        None,
        None,
        Some(23),
        None,
        Some(27),
        Some(29),
        Some(33),
        Some(37),
        None,
        None,
        None,
        None,
        None,
        None,
        Some(48),
        None,
        None,
        None,
        Some(46),
        None,
        None,
        None,
        Some(42),
        Some(38),
        Some(34),
        Some(32),
        None,
        None,
        None,
        None,
        Some(19),
    ]);

    assert_eq!(false, Solution::is_even_odd_tree(example));
}

struct Solution;

fn main() {}
