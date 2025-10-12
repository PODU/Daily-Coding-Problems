// Day 416: Min king-moves to visit points in order = sum of Chebyshev distances max(|dx|,|dy|).
// Time O(n), Space O(1).
package main

import "fmt"

func abs(a int) int {
	if a < 0 {
		return -a
	}
	return a
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
	fmt.Println(minSteps(pts))
}
