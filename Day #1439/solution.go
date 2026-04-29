// Day 1439: Find a word in a char grid reading left-to-right or top-to-bottom.
// Approach: build each row and column string, check if target is a substring.
// Time: O(R*C*L) substring scan, Space: O(R+C).
package main

import (
	"fmt"
	"strings"
)

func findWord(grid [][]byte, target string) bool {
	rows := len(grid)
	if rows == 0 {
		return false
	}
	cols := len(grid[0])
	for _, row := range grid {
		if strings.Contains(string(row), target) {
			return true
		}
	}
	for c := 0; c < cols; c++ {
		var col strings.Builder
		for r := 0; r < rows; r++ {
			col.WriteByte(grid[r][c])
		}
		if strings.Contains(col.String(), target) {
			return true
		}
	}
	return false
}

func main() {
	grid := [][]byte{
		[]byte("FACI"),
		[]byte("OBQP"),
		[]byte("ANOB"),
		[]byte("MASS"),
	}
	fmt.Println(findWord(grid, "FOAM")) // true
	fmt.Println(findWord(grid, "MASS")) // true
}
