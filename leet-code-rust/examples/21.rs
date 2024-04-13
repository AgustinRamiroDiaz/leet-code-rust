// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn from(array: &[i32]) -> Option<Box<Self>> {
        let mut out = None;
        for i in array.iter().rev() {
            out = Some(Box::new(Self { val: *i, next: out }))
        }

        out
    }
}
impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if list1.is_none() {
            return list2;
        }
        if list2.is_none() {
            return list1;
        }

        if list1.as_ref().unwrap().val < list2.as_ref().unwrap().val {
            Some(Box::new(ListNode {
                val: list1.as_ref().unwrap().val,
                next: Self::merge_two_lists(list1.unwrap().next, list2),
            }))
        } else {
            Some(Box::new(ListNode {
                val: list2.as_ref().unwrap().val,
                next: Self::merge_two_lists(list1, list2.unwrap().next),
            }))
        }
    }
}

#[test]
fn case1() {
    let expected = ListNode::from(&[1, 1, 2, 3, 4, 4]);
    let left = ListNode::from(&[1, 2, 4]);
    let right = ListNode::from(&[1, 3, 4]);
    let result = Solution::merge_two_lists(left, right);
    assert_eq!(expected, result)
}
#[test]
fn case2() {
    let expected = ListNode::from(&[0]);
    let left = ListNode::from(&[]);
    let right = ListNode::from(&[0]);
    let result = Solution::merge_two_lists(left, right);
    assert_eq!(expected, result)
}
#[test]
fn case3() {
    let expected = ListNode::from(&[]);
    let left = ListNode::from(&[]);
    let right = ListNode::from(&[]);
    let result = Solution::merge_two_lists(left, right);
    assert_eq!(expected, result)
}
struct Solution;

fn main() {
    print!("{:?}", ListNode::from(&[1, 2, 3, 4, 5, 6]).unwrap())
}
