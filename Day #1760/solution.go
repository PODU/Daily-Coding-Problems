// Day 1760: Sliding window maximum.
// Approach: monotonic deque of indices (decreasing values). O(n) time, O(k) space.
package main

import (
	"fmt"
	"strings"
)

func maxSlidingWindow(a []int, k int) []int {
	res := make([]int, 0, len(a)-k+1)
	dq := make([]int, 0, k) // indices, values decreasing
	for i, x := range a {
		if len(dq) > 0 && dq[0] <= i-k {
			dq = dq[1:]
		}
		for len(dq) > 0 && a[dq[len(dq)-1]] <= x {
			dq = dq[:len(dq)-1]
		}
		dq = append(dq, i)
		if i >= k-1 {
			res = append(res, a[dq[0]])
		}
	}
	return res
}

func main() {
	a := []int{10, 5, 2, 7, 8, 7}
	k := 3
	res := maxSlidingWindow(a, k)
	parts := make([]string, len(res))
	for i, v := range res {
		parts[i] = fmt.Sprintf("%d", v)
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
