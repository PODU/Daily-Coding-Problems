// Sort number-strings by comparator a+b > b+a (largest concat first), join; handle all-zeros.
// Time: O(n log n * L) comparisons, Space: O(n).
package main

import (
	"fmt"
	"sort"
	"strconv"
	"strings"
)

func largestNumber(nums []int) string {
	s := make([]string, len(nums))
	for i, x := range nums {
		s[i] = strconv.Itoa(x)
	}
	sort.Slice(s, func(i, j int) bool {
		return s[i]+s[j] > s[j]+s[i]
	})
	if s[0] == "0" {
		return "0"
	}
	return strings.Join(s, "")
}

func main() {
	fmt.Println(largestNumber([]int{10, 7, 76, 415}))
}
