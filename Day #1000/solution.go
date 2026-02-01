// Day 1000: Minimum of a rotated sorted array (no duplicates).
// Binary search comparing mid with the right end. O(log N) time, O(1) space.
package main

import "fmt"

func findMin(nums []int) int {
	lo, hi := 0, len(nums)-1
	for lo < hi {
		mid := (lo + hi) / 2
		if nums[mid] > nums[hi] {
			lo = mid + 1
		} else {
			hi = mid
		}
	}
	return nums[lo]
}

func main() {
	fmt.Println(findMin([]int{5, 7, 10, 3, 4})) // 3
}
