// Zigzag print: place each char at advancing column, row bounces 0..k-1..0.
// Build k row buffers, fill columns; rtrim each row. Time O(n*k), Space O(n*k).
package main

import (
	"fmt"
	"strings"
)

func zigzag(s string, k int) []string {
	n := len(s)
	grid := make([][]byte, k)
	for i := range grid {
		grid[i] = make([]byte, n)
		for j := range grid[i] {
			grid[i][j] = ' '
		}
	}
	row, dir := 0, 1
	for col := 0; col < n; col++ {
		grid[row][col] = s[col]
		if k > 1 {
			if row == 0 {
				dir = 1
			} else if row == k-1 {
				dir = -1
			}
			row += dir
		}
	}
	out := make([]string, k)
	for i := range grid {
		out[i] = strings.TrimRight(string(grid[i]), " ")
	}
	return out
}

func main() {
	for _, r := range zigzag("thisisazigzag", 4) {
		fmt.Println(r)
	}
}
