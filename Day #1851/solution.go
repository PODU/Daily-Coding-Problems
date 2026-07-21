// Day 1851: Zigzag string across k lines.
// Place char i at row=zigzag(i), col=i in a grid; print rows. O(n*k) build. Space O(n*k).
package main

import (
	"fmt"
	"strings"
)

func zigzag(s string, k int) []string {
	n := len(s)
	grid := make([][]byte, k)
	for r := range grid {
		grid[r] = make([]byte, n)
		for c := range grid[r] {
			grid[r][c] = ' '
		}
	}
	period := 1
	if k > 1 {
		period = 2 * (k - 1)
	}
	for i := 0; i < n; i++ {
		pos := 0
		if k > 1 {
			pos = i % period
		}
		row := pos
		if pos >= k {
			row = period - pos
		}
		grid[row][i] = s[i]
	}
	out := make([]string, k)
	for r := range grid {
		out[r] = strings.TrimRight(string(grid[r]), " ")
	}
	return out
}

func main() {
	for _, line := range zigzag("thisisazigzag", 4) {
		fmt.Println(line)
	}
}
