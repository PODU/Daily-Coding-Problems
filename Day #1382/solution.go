// LIS length via patience sorting: maintain tails[], sort.SearchInts (lower_bound) for strict increase. O(n log n) time, O(n) space.
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
	nums := []int{10, 9, 2, 5, 3, 7, 101, 18}
	fmt.Println(lengthOfLIS(nums))
}
