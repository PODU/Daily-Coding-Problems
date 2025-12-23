// Longest Increasing Subsequence (length), patience sorting.
// Maintain tails[]; binary-search insertion point for each value. O(n log n) time, O(n) space.
package main

import (
	"fmt"
	"sort"
)

func lengthOfLIS(nums []int) int {
	tails := []int{}
	for _, x := range nums {
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
	nums := []int{0, 8, 4, 12, 2, 10, 6, 14, 1, 9, 5, 13, 3, 11, 7, 15}
	fmt.Println(lengthOfLIS(nums))
}
