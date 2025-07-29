// Day 49: Maximum contiguous subarray sum (Kadane), empty subarray allowed.
// Time: O(n), Space: O(1).
package main

import "fmt"

func maxSubarray(a []int) int {
	best, cur := 0, 0 // empty subarray => 0
	for _, x := range a {
		cur += x
		if cur < 0 {
			cur = 0
		}
		if cur > best {
			best = cur
		}
	}
	return best
}

func main() {
	fmt.Println(maxSubarray([]int{34, -50, 42, 14, -5, 86}))
	fmt.Println(maxSubarray([]int{-5, -1, -8, -9}))
}
