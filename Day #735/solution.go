// Day 735: Find any peak element in O(log N).
// Approach: Binary search - move toward the larger neighbor; a peak must lie that way.
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
	a := []int{0, 2, 5, 3, 1}
	i := findPeak(a)
	fmt.Printf("Peak element: %d (index %d)\n", a[i], i) // 5 (index 2)
}
