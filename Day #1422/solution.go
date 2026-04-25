// Day 1422: largest sum of non-adjacent numbers (values may be 0/negative).
// Approach: rolling include/exclude DP; empty subset allowed (floor 0). O(n) time, O(1) space.
package main

import "fmt"

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func maxNonAdjacent(nums []int) int {
	incl, excl := 0, 0
	for _, n := range nums {
		newIncl := excl + n
		newExcl := max(incl, excl)
		incl, excl = newIncl, newExcl
	}
	return max(incl, excl)
}

func main() {
	fmt.Println(maxNonAdjacent([]int{2, 4, 6, 2, 5})) // 13
	fmt.Println(maxNonAdjacent([]int{5, 1, 1, 5}))    // 10
}
