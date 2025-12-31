// Sorted squares via two-pointer merge from both ends, filling result from the back.
// Time: O(n), Space: O(n).
package main

import (
	"fmt"
	"strconv"
	"strings"
)

func sortedSquares(nums []int) []int {
	n := len(nums)
	res := make([]int, n)
	lo, hi := 0, n-1
	for k := n - 1; k >= 0; k-- {
		l := nums[lo] * nums[lo]
		r := nums[hi] * nums[hi]
		if l > r {
			res[k] = l
			lo++
		} else {
			res[k] = r
			hi--
		}
	}
	return res
}

func main() {
	res := sortedSquares([]int{-9, -2, 0, 2, 3})
	parts := make([]string, len(res))
	for i, v := range res {
		parts[i] = strconv.Itoa(v)
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
