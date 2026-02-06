// Day 1032: Check if a matrix is Toeplitz.
// Compare each element to its top-left neighbor: m[i][j]==m[i-1][j-1]. O(rows*cols) time, O(1) space.
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
	m := [][]int{
		{1, 2, 3, 4, 8},
		{5, 1, 2, 3, 4},
		{4, 5, 1, 2, 3},
		{7, 4, 5, 1, 2},
	}
	if isToeplitz(m) {
		fmt.Println("True")
	} else {
		fmt.Println("False")
	}
}
