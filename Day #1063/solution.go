// Day 1063: Find a peak in a rotated sorted array of distinct values.
// Approach: binary search; if a[mid] < a[mid+1] go right else left. Time O(log n), Space O(1).
package main

import "fmt"

func findPeak(a []int) int {
	lo, hi := 0, len(a)-1
	for lo < hi {
		mid := (lo + hi) / 2
		if a[mid] < a[mid+1] {
			lo = mid + 1
		} else {
			hi = mid
		}
	}
	return lo // index of the peak
}

func main() {
	a := []int{3, 4, 5, 1, 2}
	idx := findPeak(a)
	fmt.Println(a[idx]) // 5
}
