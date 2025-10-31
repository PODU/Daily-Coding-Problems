// Zigzag: place char i at (zigzag-row, column i) in a k x n grid; print rows. O(n*k).
package main

import (
	"fmt"
	"strings"
)

func zigzag(s string, k int) {
	n := len(s)
	grid := make([][]byte, k)
	for i := range grid {
		grid[i] = []byte(strings.Repeat(" ", n))
	}
	row, dir := 0, 1
	if k == 1 {
		dir = 0
	}
	for i := 0; i < n; i++ {
		grid[row][i] = s[i]
		if row == 0 {
			dir = 1
		} else if row == k-1 {
			dir = -1
		}
		row += dir
	}
	for _, r := range grid {
		fmt.Println(strings.TrimRight(string(r), " "))
	}
}

func main() {
	zigzag("thisisazigzag", 4)
}
