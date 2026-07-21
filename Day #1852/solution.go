// Day 1852: Longest Increasing Subsequence (strict).
// Patience sorting: maintain tails[]; binary-search insertion point. O(n log n) time, O(n) space.
package main

import (
	"fmt"
	"sort"
)

func lis(a []int) int {
	tails := []int{}
	for _, x := range a {
		i := sort.SearchInts(tails, x)
		if i == len(tails) {
			tails = append(tails, x)
		} else {
			tails[i] = x
		}
	}
	return len(tails)
}

func main() {
	fmt.Println(lis([]int{10, 9, 2, 5, 3, 7, 101, 18})) // 4
}
