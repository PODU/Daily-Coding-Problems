// Day 1153: Min steps to visit points in order (8-directional moves).
// Sum of Chebyshev distances max(|dx|,|dy|) between consecutive points. O(n) time, O(1) space.
package main

import "fmt"

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func maxInt(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func minSteps(pts [][2]int) int {
	total := 0
	for i := 1; i < len(pts); i++ {
		total += maxInt(abs(pts[i][0]-pts[i-1][0]), abs(pts[i][1]-pts[i-1][1]))
	}
	return total
}

func main() {
	pts := [][2]int{{0, 0}, {1, 1}, {1, 2}}
	fmt.Println(minSteps(pts)) // 2
}
