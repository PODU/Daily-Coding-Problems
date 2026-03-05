// Day 1155: Sliding window maximum via monotonic decreasing deque of indices.
// Each index pushed/popped once. Time O(n), Space O(k).
package main

import "fmt"

func maxWindow(a []int, k int) []int {
	dq := []int{} // indices, values decreasing
	res := []int{}
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
	fmt.Println(maxWindow([]int{10, 5, 2, 7, 8, 7}, 3)) // [10 7 8 8]
}
