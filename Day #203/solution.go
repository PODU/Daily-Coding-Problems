// Day 203: Minimum of a rotated sorted array (no duplicates).
// Binary search: if mid > right, min is to the right; else min is at mid or left.
// Time: O(log n), Space: O(1).
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
