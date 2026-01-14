// Kadane's max subarray sum; empty subarray (0) allowed so answer is never negative. O(N) time, O(1) space.
package main

import "fmt"

func maxSubarraySum(a []int) int {
	best, cur := 0, 0
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
	fmt.Println(maxSubarraySum([]int{34, -50, 42, 14, -5, 86})) // 137
	fmt.Println(maxSubarraySum([]int{-5, -1, -8, -9}))          // 0
}
