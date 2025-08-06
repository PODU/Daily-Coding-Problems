// Longest strictly increasing subsequence via patience sorting (tails array + binary search).
// Time O(n log n), Space O(n).
package main

import (
	"fmt"
	"sort"
)

func lengthOfLIS(a []int) int {
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
	a := []int{0, 8, 4, 12, 2, 10, 6, 14, 1, 9, 5, 13, 3, 11, 7, 15}
	fmt.Println(lengthOfLIS(a))
}
