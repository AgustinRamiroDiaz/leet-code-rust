package leetcode

import "testing"

func timeRequiredToBuy(tickets []int, k int) int {
	result := 0
	for i := range k {
		result += min(tickets[i], tickets[k])
	}

	result += tickets[k]

	for i := k + 1; i < len(tickets); i++ {
		result += min(tickets[i], tickets[k]-1)
	}

	return result
}

func TestTimeRequiredToBuy(t *testing.T) {
	tests := []struct {
		name    string
		tickets []int
		k       int
		output  int
	}{
		{
			name:    "case 1",
			tickets: []int{2, 3, 2},
			k:       2,
			output:  6,
		},
		{
			name:    "case 2",
			tickets: []int{5, 1, 1, 1},
			k:       0,
			output:  8,
		},
	}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			result := timeRequiredToBuy(test.tickets, test.k)
			if result != test.output {
				t.Errorf("expected %d actual %d", test.output, result)
			}
		})
	}
}
