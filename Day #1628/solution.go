// Day 1628: Rotate N x N matrix 90 degrees clockwise in place.
// Transpose then reverse each row. Time O(N^2), Space O(1).
package main

import (
	"fmt"
	"strings"
)

func rotate(m [][]int) {
	n := len(m)
	for i := 0; i < n; i++ {
		for j := i + 1; j < n; j++ {
			m[i][j], m[j][i] = m[j][i], m[i][j]
		}
	}
	for _, row := range m {
		for a, b := 0, n-1; a < b; a, b = a+1, b-1 {
			row[a], row[b] = row[b], row[a]
		}
	}
}

func main() {
	m := [][]int{{1, 2, 3}, {4, 5, 6}, {7, 8, 9}}
	rotate(m)
	for _, row := range m {
		parts := make([]string, len(row))
		for j, v := range row {
			parts[j] = fmt.Sprintf("%d", v)
		}
		fmt.Println(strings.Join(parts, " "))
	}
}
