package leetcode

type ListNode struct {
	Val  int
	Next *ListNode
}

func mergeTwoLists(list1 *ListNode, list2 *ListNode) *ListNode {
	if list1 == nil {
		return list2
	}
	if list2 == nil {
		return list1
	}

	var currentVal int
	if list1.Val < list2.Val {
		currentVal = list1.Val
		list1 = list1.Next
	} else {
		currentVal = list2.Val
		list2 = list2.Next
	}

	return &ListNode{Val: currentVal, Next: mergeTwoLists(list1, list2)}
}
