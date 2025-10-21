// Rotate NxN matrix 90 deg clockwise in place: transpose then reverse each row.
// Time: O(n^2), Space: O(1).
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
	n := len(m)
	var lines []string
	for i := 0; i < n; i++ {
		prefix := " ["
		if i == 0 {
			prefix = "[["
		}
		parts := make([]string, n)
		for j := 0; j < n; j++ {
			parts[j] = strconv.Itoa(m[i][j])
		}
		suffix := "]"
		if i == n-1 {
			suffix = "]]"
		}
		lines = append(lines, prefix+strings.Join(parts, ", ")+suffix)
	}
	fmt.Println(strings.Join(lines, ",\n"))
}
