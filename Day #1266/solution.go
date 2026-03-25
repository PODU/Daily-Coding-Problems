// Day 1266: Arrange numbers to form the largest integer.
// Sort by custom comparator a+b vs b+a (descending). O(n log n) comparisons.
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
	if len(s) == 0 || s[0] == "0" {
		return "0"
	}
	return strings.Join(s, "")
}

func main() {
	fmt.Println(largestNumber([]int{10, 7, 76, 415}))
}
