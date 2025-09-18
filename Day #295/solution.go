// Kth row of Pascal's triangle (1-indexed) via iterative binomials in one array. O(k) space, O(k) time.
package main

import (
	"fmt"
	"strconv"
	"strings"
)

func pascalRow(k int) []int {
	n := k - 1 // 0-indexed row number
	row := make([]int, k)
	row[0] = 1
	for r := 1; r < k; r++ {
		row[r] = row[r-1] * (n - r + 1) / r
	}
	return row
}

func main() {
	row := pascalRow(5)
	parts := make([]string, len(row))
	for i, x := range row {
		parts[i] = strconv.Itoa(x)
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
