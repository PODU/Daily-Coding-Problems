// Day 1731: kth row of Pascal's triangle (1-indexed) using O(k) space.
// Binomial coefficients built in place. Time O(k), Space O(k).
package main

import (
	"fmt"
	"strings"
)

func pascalRow(k int) []int {
	row := make([]int, k)
	row[0] = 1
	for i := 1; i < k; i++ {
		row[i] = row[i-1] * (k - i) / i
	}
	return row
}

func main() {
	k := 5 // row [1,4,6,4,1]
	r := pascalRow(k)
	parts := make([]string, len(r))
	for i, v := range r {
		parts[i] = fmt.Sprintf("%d", v)
	}
	fmt.Println(strings.Join(parts, " "))
}
