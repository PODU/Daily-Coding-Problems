// Search in rotated sorted array via modified binary search. O(log n) time, O(1) space.
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
	arr := []int{13, 18, 25, 2, 8, 10}
	fmt.Println(search(arr, 8))
	miss := search(arr, 7)
	if miss == -1 {
		fmt.Println("null")
	} else {
		fmt.Println(miss)
	}
}
