// Approach: Monotonic deque of indices; front always holds the window max. O(n) time, O(k) space.
package main

import (
	"fmt"
	"strconv"
	"strings"
)

func maxSlidingWindow(nums []int, k int) []int {
	dq := []int{} // indices, decreasing values
	res := []int{}
	for i, v := range nums {
		if len(dq) > 0 && dq[0] <= i-k {
			dq = dq[1:]
		}
		for len(dq) > 0 && nums[dq[len(dq)-1]] <= v {
			dq = dq[:len(dq)-1]
		}
		dq = append(dq, i)
		if i >= k-1 {
			res = append(res, nums[dq[0]])
		}
	}
	return res
}

func main() {
	nums := []int{10, 5, 2, 7, 8, 7}
	k := 3
	res := maxSlidingWindow(nums, k)
	parts := make([]string, len(res))
	for i, v := range res {
		parts[i] = strconv.Itoa(v)
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
