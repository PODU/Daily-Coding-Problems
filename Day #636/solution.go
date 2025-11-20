// Day 636: Minimum in a rotated sorted array (no duplicates).
// Approach: binary search comparing mid with right endpoint.
// Time: O(log N), Space: O(1).
package main

import "fmt"

func findMin(a []int) int {
	lo, hi := 0, len(a)-1
	for lo < hi {
		mid := (lo + hi) / 2
		if a[mid] > a[hi] {
			lo = mid + 1
		} else {
			hi = mid
		}
	}
	return a[lo]
}

func main() {
	fmt.Println(findMin([]int{5, 7, 10, 3, 4})) // 3
}
