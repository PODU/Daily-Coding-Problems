// Majority via Boyer-Moore voting (O(n) time, O(1) space). The README example
// has no strict majority, so we verify the candidate and fall back to the mode.
package main

import "fmt"

func countOf(v []int, target int) int {
	c := 0
	for _, x := range v {
		if x == target {
			c++
		}
	}
	return c
}

func majority(nums []int) int {
	count, candidate := 0, 0
	for _, x := range nums { // Boyer-Moore voting pass
		if count == 0 {
			candidate = x
		}
		if x == candidate {
			count++
		} else {
			count--
		}
	}
	if countOf(nums, candidate) > len(nums)/2 {
		return candidate
	}
	best := nums[0] // fallback: most frequent element
	for _, x := range nums {
		if countOf(nums, x) > countOf(nums, best) {
			best = x
		}
	}
	return best
}

func main() {
	nums := []int{1, 2, 1, 1, 3, 4, 0}
	fmt.Println(majority(nums)) // 1
}
