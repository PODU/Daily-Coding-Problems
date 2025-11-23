// Day 645: Find a word in a grid going left-to-right or top-to-bottom.
// Approach: scan every row and every column for the target as a substring start.
// Time: O(R*C*L), Space: O(1).
package main

import "fmt"

func findWord(grid [][]byte, word string) bool {
	R, C, L := len(grid), len(grid[0]), len(word)
	for r := 0; r < R; r++ {
		for c := 0; c+L <= C; c++ {
			ok := true
			for k := 0; k < L; k++ {
				if grid[r][c+k] != word[k] {
					ok = false
					break
				}
			}
			if ok {
				return true
			}
		}
	}
	for c := 0; c < C; c++ {
		for r := 0; r+L <= R; r++ {
			ok := true
			for k := 0; k < L; k++ {
				if grid[r+k][c] != word[k] {
					ok = false
					break
				}
			}
			if ok {
				return true
			}
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
