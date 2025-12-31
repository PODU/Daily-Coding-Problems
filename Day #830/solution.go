// Day 830: Largest number formed by concatenating the given numbers.
// Sort strings by comparator a+b > b+a (descending). O(N log N) compares of O(L) strings.
package main

import (
	"fmt"
	"sort"
	"strconv"
	"strings"
)

func largestNumber(nums []int64) string {
	strs := make([]string, len(nums))
	for i, n := range nums {
		strs[i] = strconv.FormatInt(n, 10)
	}
	sort.Slice(strs, func(i, j int) bool {
		return strs[i]+strs[j] > strs[j]+strs[i]
	})
	result := strings.Join(strs, "")
	if len(result) > 0 && result[0] == '0' {
		return "0"
	}
	return result
}

func main() {
	fmt.Println(largestNumber([]int64{10, 7, 76, 415})) // 77641510
}
