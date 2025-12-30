// Fixed point in sorted distinct array via binary search (nums[i]-i non-decreasing).
// Time: O(log n), Space: O(1).
package main

import "fmt"

func fixedPoint(nums []int) int {
	lo, hi := 0, len(nums)-1
	for lo <= hi {
		mid := (lo + hi) / 2
		if nums[mid] == mid {
			return mid
		} else if nums[mid] < mid {
			lo = mid + 1
		} else {
			hi = mid - 1
		}
	}
	return -1
}

func main() {
	r1 := fixedPoint([]int{-6, 0, 2, 40})
	if r1 == -1 {
		fmt.Println("False")
	} else {
		fmt.Println(r1)
	}
	r2 := fixedPoint([]int{1, 5, 7, 8})
	if r2 == -1 {
		fmt.Println("False")
	} else {
		fmt.Println(r2)
	}
}
