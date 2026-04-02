// Day 1285: Print a string in zigzag form across k lines.
// Place char i at column i, row = triangle-wave of i. Time O(n*k) to render, Space O(n*k).
package main

import (
	"fmt"
	"strings"
)

func zigzag(s string, k int) {
	n := len(s)
	if k <= 1 {
		fmt.Println(s)
		return
	}
	period := 2 * (k - 1)
	grid := make([][]byte, k)
	for r := range grid {
		grid[r] = []byte(strings.Repeat(" ", n))
	}
	for i := 0; i < n; i++ {
		pos := i % period
		row := pos
		if pos >= k {
			row = period - pos
		}
		grid[row][i] = s[i]
	}
	for _, row := range grid {
		fmt.Println(strings.TrimRight(string(row), " "))
	}
}

func main() {
	zigzag("thisisazigzag", 4)
}
