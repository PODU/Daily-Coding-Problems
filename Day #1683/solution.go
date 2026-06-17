// Day 1683: Find word reading left-to-right (a row) or top-to-bottom (a column).
// Build row/column strings, substring search. Time O(N*M), Space O(N+M).
package main

import (
	"fmt"
	"strings"
)

func findWord(grid [][]byte, word string) bool {
	rows, cols := len(grid), len(grid[0])
	for r := 0; r < rows; r++ {
		if strings.Contains(string(grid[r]), word) {
			return true
		}
	}
	for c := 0; c < cols; c++ {
		var col strings.Builder
		for r := 0; r < rows; r++ {
			col.WriteByte(grid[r][c])
		}
		if strings.Contains(col.String(), word) {
			return true
		}
	}
	return false
}

func main() {
	grid := [][]byte{
		[]byte("FACI"), []byte("OBQP"), []byte("ANOB"), []byte("MASS")}
	fmt.Println("'FOAM' ->", findWord(grid, "FOAM")) // true
	fmt.Println("'MASS' ->", findWord(grid, "MASS")) // true
}
