// Day 194: Segments p_i->q_i cross iff order of p and q disagree. Count = inversions of q
// after sorting pairs by p. Merge-sort inversion count. Time O(n log n), Space O(n).
package main

import (
	"fmt"
	"sort"
)

func mergeCount(v []int) int64 {
	n := len(v)
	if n <= 1 {
		return 0
	}
	m := n / 2
	left := append([]int(nil), v[:m]...)
	right := append([]int(nil), v[m:]...)
	cnt := mergeCount(left) + mergeCount(right)
	i, j, k := 0, 0, 0
	for i < len(left) && j < len(right) {
		if left[i] <= right[j] {
			v[k] = left[i]
			i++
		} else {
			v[k] = right[j]
			j++
			cnt += int64(len(left) - i)
		}
		k++
	}
	for i < len(left) {
		v[k] = left[i]
		i++
		k++
	}
	for j < len(right) {
		v[k] = right[j]
		j++
		k++
	}
	return cnt
}

func countCrossings(p, q []int) int64 {
	n := len(p)
	idx := make([]int, n)
	for i := range idx {
		idx[i] = i
	}
	sort.Slice(idx, func(a, b int) bool { return p[idx[a]] < p[idx[b]] })
	qs := make([]int, n)
	for k, i := range idx {
		qs[k] = q[i]
	}
	return mergeCount(qs)
}

func main() {
	fmt.Println(countCrossings([]int{1, 2, 3, 4}, []int{4, 3, 2, 1}))
}
