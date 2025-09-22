// Day 315: Toeplitz check - each cell equals its top-left neighbor. O(N*M) time, O(1) space.
package main

import "fmt"

func isToeplitz(m [][]int) bool {
	for i := 1; i < len(m); i++ {
		for j := 1; j < len(m[i]); j++ {
			if m[i][j] != m[i-1][j-1] {
				return false
			}
		}
	}
	return true
}

func main() {
	m := [][]int{{1, 2, 3, 4, 8}, {5, 1, 2, 3, 4}, {4, 5, 1, 2, 3}, {7, 4, 5, 1, 2}}
	if isToeplitz(m) {
		fmt.Println("True")
	} else {
		fmt.Println("False")
	}
}
