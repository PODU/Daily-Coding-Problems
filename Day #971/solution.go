// Day 971: Rotate N x N matrix 90 degrees clockwise in place.
// Approach: transpose then reverse each row. Time O(N^2), Space O(1).
package main

import (
	"fmt"
	"strconv"
	"strings"
)

func rotate(m [][]int) {
	n := len(m)
	for i := 0; i < n; i++ {
		for j := i + 1; j < n; j++ {
			m[i][j], m[j][i] = m[j][i], m[i][j]
		}
	}
	for i := 0; i < n; i++ {
		for l, r := 0, n-1; l < r; l, r = l+1, r-1 {
			m[i][l], m[i][r] = m[i][r], m[i][l]
		}
	}
}

func main() {
	m := [][]int{{1, 2, 3}, {4, 5, 6}, {7, 8, 9}}
	rotate(m)
	for _, row := range m {
		parts := make([]string, len(row))
		for j, v := range row {
			parts[j] = strconv.Itoa(v)
		}
		fmt.Println(strings.Join(parts, " "))
	}
}
