// Longest strictly Increasing Subsequence length via patience sorting + binary search.
// Time O(N log N), Space O(N). Also reconstructs one valid LIS.
package main

import (
	"fmt"
	"sort"
)

func lisLength(nums []int) (int, []int) {
	tails := []int{}    // value of smallest tail per length
	tailsIdx := []int{} // index in nums of that tail
	prev := make([]int, len(nums))
	for i := range prev {
		prev[i] = -1
	}
	for i, x := range nums {
		pos := sort.SearchInts(tails, x) // lower_bound
		if pos == len(tails) {
			tails = append(tails, x)
			tailsIdx = append(tailsIdx, i)
		} else {
			tails[pos] = x
			tailsIdx[pos] = i
		}
		if pos > 0 {
			prev[i] = tailsIdx[pos-1]
		} else {
			prev[i] = -1
		}
	}
	seq := []int{}
	k := -1
	if len(tailsIdx) > 0 {
		k = tailsIdx[len(tailsIdx)-1]
	}
	for k != -1 {
		seq = append(seq, nums[k])
		k = prev[k]
	}
	// reverse
	for l, r := 0, len(seq)-1; l < r; l, r = l+1, r-1 {
		seq[l], seq[r] = seq[r], seq[l]
	}
	return len(tails), seq
}

func main() {
	nums := []int{0, 8, 4, 12, 2, 10, 6, 14, 1, 9, 5, 13, 3, 11, 7, 15}
	length, seq := lisLength(nums)
	fmt.Println(length)
	fmt.Println(seq)
}
