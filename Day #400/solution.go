// Prefix-sum array P (P[k]=sum of first k elems); sum(i,j)=P[j]-P[i]. Preprocess O(n), query O(1).
package main

import "fmt"

type RangeSum struct {
	P []int
}

func NewRangeSum(L []int) *RangeSum {
	p := make([]int, len(L)+1)
	for k, x := range L {
		p[k+1] = p[k] + x
	}
	return &RangeSum{P: p}
}

func (r *RangeSum) Sum(i, j int) int {
	return r.P[j] - r.P[i]
}

func main() {
	L := []int{1, 2, 3, 4, 5}
	rs := NewRangeSum(L)
	fmt.Println(rs.Sum(1, 3))
}
