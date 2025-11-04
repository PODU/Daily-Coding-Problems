// Count columns that are NOT non-decreasing top-to-bottom; that's the min to remove.
// O(N*M) time, O(1) extra space.
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
	fmt.Println(minDeletions([]string{"cba", "daf", "ghi"})) // 1
	fmt.Println(minDeletions([]string{"abcdef"}))            // 0
	fmt.Println(minDeletions([]string{"zyx", "wvu", "tsr"})) // 3
}
