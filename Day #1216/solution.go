// Day 1216: Min columns to delete so each column is non-decreasing top->bottom.
// Approach: scan each column once, count unsorted columns. Time O(N*M), Space O(1).
package main

import "fmt"

func minDeletions(grid []string) int {
	if len(grid) == 0 {
		return 0
	}
	rows, cols, count := len(grid), len(grid[0]), 0
	for c := 0; c < cols; c++ {
		for r := 1; r < rows; r++ {
			if grid[r][c] < grid[r-1][c] {
				count++
				break
			}
		}
	}
	return count
}

func main() {
	grid := []string{"cba", "daf", "ghi"}
	fmt.Println(minDeletions(grid)) // 1
}
