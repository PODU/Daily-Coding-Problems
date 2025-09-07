// Largest number: sort by comparator a+b > b+a (concatenation), then join.
// Time: O(n log n * L), Space: O(n * L).
package main

import (
	"fmt"
	"sort"
	"strconv"
	"strings"
)

func largestNumber(nums []int) string {
	s := make([]string, len(nums))
	for i, n := range nums {
		s[i] = strconv.Itoa(n)
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
	fmt.Println(largestNumber([]int{10, 7, 76, 415})) // 77641510
}
