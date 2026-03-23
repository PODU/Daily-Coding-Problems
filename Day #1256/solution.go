// Max contiguous subarray sum, empty allowed: Kadane, clamp running sum at 0.
// Time O(n), Space O(1).
package main

import "fmt"

func maxSub(a []int) int {
	cur, best := 0, 0
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
	fmt.Println(maxSub([]int{34, -50, 42, 14, -5, 86}))
	fmt.Println(maxSub([]int{-5, -1, -8, -9}))
}
