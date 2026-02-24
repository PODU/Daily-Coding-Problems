// Day 1120 - Minimum steps to cover points in order (8-directional moves)
// Cost between two points is Chebyshev distance max(|dx|,|dy|); sum them.
// Time: O(n), Space: O(1).
package main

import "fmt"

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func maxi(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func minSteps(points [][2]int) int {
	total := 0
	for i := 1; i < len(points); i++ {
		total += maxi(abs(points[i][0]-points[i-1][0]), abs(points[i][1]-points[i-1][1]))
	}
	return total
}

func main() {
	points := [][2]int{{0, 0}, {1, 1}, {1, 2}}
	fmt.Println(minSteps(points)) // 2
}
