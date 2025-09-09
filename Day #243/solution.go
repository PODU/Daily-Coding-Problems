// Split array into k parts minimizing max partition sum.
// Binary search on answer in [max, sum], greedy feasibility check. O(n log(sum)).
package main

import "fmt"

func canSplit(nums []int, k int, cap int) bool {
	parts, cur := 1, 0
	for _, x := range nums {
		if cur+x > cap {
			parts++
			cur = x
		} else {
			cur += x
		}
	}
	return parts <= k
}

func splitArray(nums []int, k int) int {
	lo, hi := 0, 0
	for _, x := range nums {
		if x > lo {
			lo = x
		}
		hi += x
	}
	for lo < hi {
		mid := lo + (hi-lo)/2
		if canSplit(nums, k, mid) {
			hi = mid
		} else {
			lo = mid + 1
		}
	}
	return lo
}

func main() {
	nums := []int{5, 1, 2, 7, 3, 4}
	fmt.Println(splitArray(nums, 3))
}
