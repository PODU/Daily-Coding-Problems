// Day 564: Largest sum of non-adjacent numbers.
// DP tracking incl/excl running maxes. Time O(n), Space O(1).
package main

import "fmt"

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func largestNonAdjacent(nums []int) int {
	incl, excl := 0, 0 // best sums including / excluding previous element
	for _, x := range nums {
		incl, excl = excl+x, max(incl, excl)
	}
	return max(incl, excl)
}

func main() {
	fmt.Println(largestNonAdjacent([]int{2, 4, 6, 2, 5}))
	fmt.Println(largestNonAdjacent([]int{5, 1, 1, 5}))
}
