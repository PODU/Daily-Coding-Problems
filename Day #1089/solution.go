// Sliding window maximum via monotonic decreasing deque of indices. Time O(n), Space O(k).
package main

import "fmt"

func maxSlidingWindow(a []int, k int) []int {
	dq := []int{} // indices, values decreasing
	res := []int{}
	for i := 0; i < len(a); i++ {
		if len(dq) > 0 && dq[0] <= i-k {
			dq = dq[1:]
		}
		for len(dq) > 0 && a[dq[len(dq)-1]] <= a[i] {
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
	fmt.Println(maxSlidingWindow(a, 3))
}
