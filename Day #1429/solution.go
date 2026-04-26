// Day 1429: Find a peak element (greater than both neighbors) in O(log N).
// Approach: binary search; if a[mid] < a[mid+1] a peak lies right, else left/at mid.
// Time: O(log n), Space: O(1).
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
	return lo
}

func main() {
	a := []int{1, 3, 5, 7, 6, 4, 2}
	fmt.Println(a[findPeak(a)]) // 7
}
