// Day 1723: Search element in rotated sorted array.
// Modified binary search: one half is always sorted; decide which side to recurse.
// Time: O(log n), Space: O(1). Returns index, or -1 if absent.
package main

import "fmt"

func search(a []int, target int) int {
	lo, hi := 0, len(a)-1
	for lo <= hi {
		mid := lo + (hi-lo)/2
		if a[mid] == target {
			return mid
		}
		if a[lo] <= a[mid] { // left half sorted
			if a[lo] <= target && target < a[mid] {
				hi = mid - 1
			} else {
				lo = mid + 1
			}
		} else { // right half sorted
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
	for _, target := range []int{8, 99} {
		if i := search(a, target); i >= 0 {
			fmt.Println(i) // 4
		} else {
			fmt.Println("nil") // not found, per the problem statement
		}
	}
}
