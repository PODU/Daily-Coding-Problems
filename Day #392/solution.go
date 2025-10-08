// Island perimeter: +4 per land cell, -2 per adjacent right/down land pair. O(R*C) time, O(1) space.
package main

import "fmt"

func islandPerimeter(g [][]int) int {
	R := len(g)
	C := 0
	if R > 0 {
		C = len(g[0])
	}
	per := 0
	for r := 0; r < R; r++ {
		for c := 0; c < C; c++ {
			if g[r][c] == 1 {
				per += 4
				if c+1 < C && g[r][c+1] == 1 {
					per -= 2
				}
				if r+1 < R && g[r+1][c] == 1 {
					per -= 2
				}
			}
		}
	}
	return per
}

func main() {
	grid := [][]int{{0, 1, 1, 0}, {1, 1, 1, 0}, {0, 1, 1, 0}, {0, 0, 1, 0}}
	fmt.Println(islandPerimeter(grid))
}
