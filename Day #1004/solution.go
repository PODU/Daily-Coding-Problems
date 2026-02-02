// Day 1004: Range sum query sum(i, j) = L[i:j].
// Pre-process a prefix-sum array (O(N)); each query is prefix[j]-prefix[i] in O(1).
package main

import "fmt"

type RangeSum struct {
	prefix []int
}

func NewRangeSum(L []int) *RangeSum {
	prefix := make([]int, len(L)+1)
	for i, v := range L {
		prefix[i+1] = prefix[i] + v
	}
	return &RangeSum{prefix}
}

func (r *RangeSum) Sum(i, j int) int {
	return r.prefix[j] - r.prefix[i]
}

func main() {
	rs := NewRangeSum([]int{1, 2, 3, 4, 5})
	fmt.Println(rs.Sum(1, 3)) // 5
}
