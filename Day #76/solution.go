// Day 76: Minimum columns to remove so every column is sorted top-to-bottom.
// Greedy scan: count columns that are not non-decreasing. Time O(N*M), Space O(1).
package main

import "fmt"

func minColumnsToRemove(grid []string) int {
	if len(grid) == 0 {
		return 0
	}
	rows, cols := len(grid), len(grid[0])
	remove := 0
	for c := 0; c < cols; c++ {
		for r := 1; r < rows; r++ {
			if grid[r][c] < grid[r-1][c] {
				remove++
				break
			}
		}
	}
	return remove
}

func main() {
	grid := []string{"cba", "daf", "ghi"}
	fmt.Println(minColumnsToRemove(grid)) // 1
}
