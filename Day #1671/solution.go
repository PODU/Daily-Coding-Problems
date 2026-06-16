// Day 1671: Min columns to remove so each column is non-decreasing top->bottom.
// Count columns containing any out-of-order adjacent pair. Time O(N*M), Space O(1).
package main

import "fmt"

func minDeletions(g []string) int {
	if len(g) == 0 {
		return 0
	}
	rows, cols, del := len(g), len(g[0]), 0
	for j := 0; j < cols; j++ {
		for i := 0; i+1 < rows; i++ {
			if g[i][j] > g[i+1][j] {
				del++
				break
			}
		}
	}
	return del
}

func main() {
	fmt.Println(minDeletions([]string{"cba", "daf", "ghi"})) // 1
}
