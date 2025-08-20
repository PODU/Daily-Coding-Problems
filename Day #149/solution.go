// Range sum via prefix sums: O(n) preprocessing, O(1) per query. sum(i,j) = pre[j]-pre[i].
package main

import "fmt"

type RangeSum struct {
	pre []int
}

func NewRangeSum(L []int) *RangeSum {
	pre := make([]int, len(L)+1)
	for k, v := range L {
		pre[k+1] = pre[k] + v
	}
	return &RangeSum{pre: pre}
}

func (rs *RangeSum) Sum(i, j int) int {
	return rs.pre[j] - rs.pre[i]
}

func main() {
	L := []int{1, 2, 3, 4, 5}
	rs := NewRangeSum(L)
	fmt.Println(rs.Sum(1, 3))
}
