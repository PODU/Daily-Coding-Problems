// Longest Increasing Subsequence via patience sorting: maintain a "tails" slice and
// binary-search the insertion point for each element. Time O(n log n), Space O(n).
package main

import (
	"fmt"
	"sort"
)

func lengthOfLIS(nums []int) int {
	tails := []int{} // tails[i] = smallest tail of an increasing subseq of length i+1
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
	nums := []int{10, 9, 2, 5, 3, 7, 101, 18}
	fmt.Println("Input:", nums)
	fmt.Println("LIS length:", lengthOfLIS(nums)) // 4
}
