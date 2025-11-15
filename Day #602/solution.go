// Approach: sort segments by p, then count inversions in the q-order via merge sort.
// Two segments cross iff their p-order and q-order disagree => an inversion. Time O(n log n), Space O(n).
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
	left, il := mergeCount(a[:m])
	right, ir := mergeCount(a[m:])
	merged := make([]int, 0, len(a))
	inv := il + ir
	i, j := 0, 0
	for i < len(left) && j < len(right) {
		if left[i] <= right[j] {
			merged = append(merged, left[i])
			i++
		} else {
			merged = append(merged, right[j])
			j++
			inv += int64(len(left) - i)
		}
	}
	merged = append(merged, left[i:]...)
	merged = append(merged, right[j:]...)
	return merged, inv
}

func countIntersections(p, q []int) int64 {
	order := make([]int, len(p))
	for i := range order {
		order[i] = i
	}
	sort.Slice(order, func(x, y int) bool { return p[order[x]] < p[order[y]] })
	qs := make([]int, len(p))
	for i, k := range order {
		qs[i] = q[k]
	}
	_, inv := mergeCount(qs)
	return inv
}

func main() {
	p := []int{1, 2, 3, 4}
	q := []int{4, 3, 2, 1}
	fmt.Println(countIntersections(p, q))
}
