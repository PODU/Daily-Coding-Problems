// Maximum contiguous subarray sum (Kadane), empty subarray allowed (min 0).
// Time O(n), Space O(1).
package main

import "fmt"

func maxSubarray(a []int) int {
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
	fmt.Println(maxSubarray([]int{34, -50, 42, 14, -5, 86}))
}
