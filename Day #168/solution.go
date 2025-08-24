// Rotate NxN 90 clockwise in place: transpose then reverse each row. O(n^2) time, O(1) extra space.
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
		for a, b := 0, n-1; a < b; a, b = a+1, b-1 {
			m[i][a], m[i][b] = m[i][b], m[i][a]
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
		fmt.Println("[" + strings.Join(parts, ", ") + "]")
	}
}
