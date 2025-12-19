// Day 763: Sliding window maximum via a monotonic decreasing deque of indices.
// Time: O(n), Space: O(k). Prints each window max as it is computed.
package main

import "fmt"

func slidingMax(a []int, k int) {
	dq := []int{} // indices, values decreasing
	out := []int{}
	for i, x := range a {
		for len(dq) > 0 && a[dq[len(dq)-1]] <= x {
			dq = dq[:len(dq)-1]
		}
		dq = append(dq, i)
		if dq[0] <= i-k {
			dq = dq[1:]
		}
		if i >= k-1 {
			out = append(out, a[dq[0]])
		}
	}
	fmt.Println(out)
}

func main() {
	slidingMax([]int{10, 5, 2, 7, 8, 7}, 3) // [10 7 8 8]
}
