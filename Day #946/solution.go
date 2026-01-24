// Day 946: kth row of Pascal's triangle (1-indexed) using O(k) space.
// In-place update of a single row, right-to-left. Time O(k^2), Space O(k).
package main

import (
	"fmt"
	"strconv"
	"strings"
)

func pascalRow(k int) []int64 {
	row := make([]int64, k)
	row[0] = 1
	for i := 1; i < k; i++ {
		for j := i; j > 0; j-- {
			row[j] += row[j-1]
		}
	}
	return row
}

func main() {
	k := 5 // README example -> 5th row
	r := pascalRow(k)
	parts := make([]string, len(r))
	for i, v := range r {
		parts[i] = strconv.FormatInt(v, 10)
	}
	fmt.Println(strings.Join(parts, " "))
}
