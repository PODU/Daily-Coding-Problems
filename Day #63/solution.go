// Search word in matrix rows (L->R) and columns (top->bottom) via substring check.
// Time O(N*M*L), Space O(max(N,M)).
package main

import (
	"fmt"
	"strings"
)

func findWord(grid [][]byte, word string) bool {
	n, m := len(grid), len(grid[0])
	for r := 0; r < n; r++ {
		if strings.Contains(string(grid[r]), word) {
			return true
		}
	}
	for c := 0; c < m; c++ {
		var col strings.Builder
		for r := 0; r < n; r++ {
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
		{'F', 'A', 'C', 'I'},
		{'O', 'B', 'Q', 'P'},
		{'A', 'N', 'O', 'B'},
		{'M', 'A', 'S', 'S'},
	}
	fmt.Printf("'FOAM' -> %t\n", findWord(grid, "FOAM"))
	fmt.Printf("'MASS' -> %t\n", findWord(grid, "MASS"))
}
