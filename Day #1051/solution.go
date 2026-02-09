// Search in rotated sorted array via modified binary search.
// Time: O(log n), Space: O(1).
package main

import "fmt"

// returns index, or -1 representing null
func searchRotated(a []int, target int) int {
	lo, hi := 0, len(a)-1
	for lo <= hi {
		mid := lo + (hi-lo)/2
		if a[mid] == target {
			return mid
		}
		if a[lo] <= a[mid] {
			if a[lo] <= target && target < a[mid] {
				hi = mid - 1
			} else {
				lo = mid + 1
			}
		} else {
			if a[mid] < target && target <= a[hi] {
				lo = mid + 1
			} else {
				hi = mid - 1
			}
		}
	}
	return -1
}

func main() {
	a := []int{13, 18, 25, 2, 8, 10}
	idx := searchRotated(a, 8)
	if idx < 0 {
		fmt.Println("null")
	} else {
		fmt.Println(idx) // 4
	}
}
