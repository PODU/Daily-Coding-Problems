// Count intersecting segment pairs: sort segments by p, then count inversions in q.
// Two segments cross iff their p-order and q-order disagree. O(n log n) time, O(n) space.
package main

import (
	"fmt"
	"sort"
)

func mergeCount(a []int) ([]int, int64) {
	if len(a) <= 1 {
		return a, 0
	}
	m := len(a) / 2
	left, cl := mergeCount(a[:m])
	right, cr := mergeCount(a[m:])
	merged := make([]int, 0, len(a))
	i, j := 0, 0
	c := cl + cr
	for i < len(left) && j < len(right) {
		if left[i] <= right[j] {
			merged = append(merged, left[i])
			i++
		} else {
			merged = append(merged, right[j])
			j++
			c += int64(len(left) - i)
		}
	}
	merged = append(merged, left[i:]...)
	merged = append(merged, right[j:]...)
	return merged, c
}

func countIntersections(p, q []int) int64 {
	n := len(p)
	order := make([]int, n)
	for k := range order {
		order[k] = k
	}
	sort.Slice(order, func(x, y int) bool { return p[order[x]] < p[order[y]] })
	qq := make([]int, n)
	for k := 0; k < n; k++ {
		qq[k] = q[order[k]]
	}
	_, c := mergeCount(qq)
	return c
}

func main() {
	p, q := []int{1, 2, 3}, []int{3, 1, 2}
	fmt.Println(countIntersections(p, q)) // 2
}
