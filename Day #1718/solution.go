// Day 1718: Count intersecting segment pairs (p_i on y=0 -> q_i on y=1).
// Two segments cross iff their (p, q) ordering is inverted: sort by p,
// count inversions in q via merge sort. Time: O(n log n), Space: O(n).
package main

import (
	"fmt"
	"sort"
)

func mergeCount(a []int, l, r int) int64 {
	if r-l <= 1 {
		return 0
	}
	mid := (l + r) / 2
	inv := mergeCount(a, l, mid) + mergeCount(a, mid, r)
	tmp := make([]int, 0, r-l)
	i, j := l, mid
	for i < mid && j < r {
		if a[i] <= a[j] {
			tmp = append(tmp, a[i])
			i++
		} else {
			inv += int64(mid - i)
			tmp = append(tmp, a[j])
			j++
		}
	}
	for i < mid {
		tmp = append(tmp, a[i])
		i++
	}
	for j < r {
		tmp = append(tmp, a[j])
		j++
	}
	copy(a[l:r], tmp)
	return inv
}

func countIntersections(p, q []int) int64 {
	n := len(p)
	idx := make([]int, n)
	for i := range idx {
		idx[i] = i
	}
	sort.Slice(idx, func(x, y int) bool { return p[idx[x]] < p[idx[y]] })
	qs := make([]int, n)
	for i := 0; i < n; i++ {
		qs[i] = q[idx[i]]
	}
	return mergeCount(qs, 0, n)
}

func main() {
	p := []int{1, 2, 3, 4}
	q := []int{2, 1, 4, 3}
	fmt.Printf("Intersecting pairs: %d\n", countIntersections(p, q))
}
