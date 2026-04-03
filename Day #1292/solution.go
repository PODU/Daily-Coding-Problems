// Day 1292: kth (0-indexed) row of Pascal's triangle.
// Update row in place from right to left. O(k^2) time, O(k) space.
package main

import (
	"fmt"
	"strings"
)

func pascalRow(k int) []int {
	row := make([]int, k+1)
	for i := range row {
		row[i] = 1
	}
	for i := 1; i <= k; i++ {
		for j := i - 1; j >= 1; j-- {
			row[j] += row[j-1]
		}
	}
	return row
}

func main() {
	row := pascalRow(4) // 1 4 6 4 1
	parts := make([]string, len(row))
	for i, v := range row {
		parts[i] = fmt.Sprintf("%d", v)
	}
	fmt.Println(strings.Join(parts, " "))
}
