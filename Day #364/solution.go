// Day 364: Longest strictly increasing subsequence length.
// Patience sorting: keep tails[], binary-search first tail >= x and replace.
// Time O(n log n), Space O(n).
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
