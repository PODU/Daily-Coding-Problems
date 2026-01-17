// Count columns to delete so each remaining column is non-decreasing top->bottom.
// Scan each column for any adjacent out-of-order pair. Time O(N*M), Space O(1).
package main

import "fmt"

func minDeletions(grid []string) int {
	if len(grid) == 0 {
		return 0
	}
	n, m, count := len(grid), len(grid[0]), 0
	for c := 0; c < m; c++ {
		for i := 0; i+1 < n; i++ {
			if grid[i][c] > grid[i+1][c] {
				count++
				break
			}
		}
	}
	return count
}

func main() {
	grid := []string{"cba", "daf", "ghi"}
	fmt.Println(minDeletions(grid))
}
