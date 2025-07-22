// Max sum of non-adjacent numbers: track best-including vs best-excluding current.
// Time: O(n), Space: O(1). (Empty pick allowed, so negatives can be skipped.)
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
		ni := excl + n
		ne := max(incl, excl)
		incl, excl = ni, ne
	}
	return max(incl, excl)
}

func main() {
	fmt.Println(maxNonAdjacent([]int{2, 4, 6, 2, 5}))
}
