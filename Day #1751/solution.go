// Day 1751: Min steps to visit points in order on an 8-directional grid.
// Sum of Chebyshev distances max(|dx|,|dy|) between consecutive points. O(n) time, O(1) space.
package main

import "fmt"

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func minSteps(pts [][2]int) int {
	total := 0
	for i := 1; i < len(pts); i++ {
		dx := abs(pts[i][0] - pts[i-1][0])
		dy := abs(pts[i][1] - pts[i-1][1])
		total += max(dx, dy)
	}
	return total
}

func main() {
	pts := [][2]int{{0, 0}, {1, 1}, {1, 2}}
	fmt.Println(minSteps(pts)) // 2
}
