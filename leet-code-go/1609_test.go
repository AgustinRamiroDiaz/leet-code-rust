package leetcode

import "testing"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func is_sorted(row []int, strictly_increasing bool, filter func(int) bool) bool {
	if len(row) == 0 {
		return true
	}

	if !filter(row[0]) {
		return false
	}

	for index := 1; index < len(row); index++ {
		val := row[index]
		if !filter(val) {
			return false
		}
		if (strictly_increasing && val <= row[index-1]) || (!strictly_increasing && val >= row[index-1]) {
			return false
		}
	}

	return true
}

func isEvenOddTree(root *TreeNode) bool {
	current_row := []*TreeNode{}
	row_index := 0

	current_row = []*TreeNode{}

	if root != nil {
		current_row = append(current_row, root)
	}

	for len(current_row) != 0 {
		current_values := []int{}
		for _, node := range current_row {
			current_values = append(current_values, node.Val)
		}

		if !is_sorted(current_values, row_index%2 == 0, func(i int) bool {
			return i%2 != row_index%2
		}) {
			return false
		}

		row_index += 1
		new_current_row := []*TreeNode{}
		for _, node := range current_row {
			for _, node := range []*TreeNode{node.Left, node.Right} {
				if node != nil {
					new_current_row = append(new_current_row, node)
				}
			}
		}

		current_row = new_current_row
	}

	return true
}

func TestIsEvenOddTree(t *testing.T) {
	// input := [2,12,8,5,9,null,null,18,16]

	input := &TreeNode{
		Val: 2,
		Left: &TreeNode{
			Val: 12,
			Left: &TreeNode{
				Val:   5,
				Left:  &TreeNode{Val: 18, Left: nil, Right: nil},
				Right: &TreeNode{Val: 16, Left: nil, Right: nil},
			},
			Right: &TreeNode{
				Val:   9,
				Left:  nil,
				Right: nil,
			},
		},
		Right: &TreeNode{
			Val:   8,
			Left:  nil,
			Right: nil,
		},
	}

	result := isEvenOddTree(input)

	if result != false {
		t.Errorf("expected %t, got %t", false, result)
	}
}
