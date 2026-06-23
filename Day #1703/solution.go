// Prefix sums: precompute prefix[k]=sum(L[0:k]); sum(i,j)=prefix[j]-prefix[i].
// Preprocess O(n), query O(1), Space O(n).
package main

import "fmt"

type RangeSum struct {
	prefix []int
}

func NewRangeSum(L []int) *RangeSum {
	prefix := make([]int, len(L)+1)
	for k, x := range L {
		prefix[k+1] = prefix[k] + x
	}
	return &RangeSum{prefix}
}

func (r *RangeSum) Sum(i, j int) int {
	return r.prefix[j] - r.prefix[i]
}

func main() {
	L := []int{1, 2, 3, 4, 5}
	rs := NewRangeSum(L)
	fmt.Println(rs.Sum(1, 3))
}
