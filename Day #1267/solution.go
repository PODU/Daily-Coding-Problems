// Day 1267: Range sum query with preprocessing.
// Prefix-sum array: O(n) preprocess, O(1) per sum(i, j) query.
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

func (r *RangeSum) Sum(i, j int) int { return r.prefix[j] - r.prefix[i] } // L[i:j]

func main() {
	rs := NewRangeSum([]int{1, 2, 3, 4, 5})
	fmt.Println(rs.Sum(1, 3))
}
