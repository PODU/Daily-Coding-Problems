// Day 100: 8-directional steps between two points = Chebyshev distance
// max(|dx|,|dy|). Sum over consecutive points. O(n) time.
package main

import "fmt"

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func minSteps(pts [][2]int) int {
	total := 0
	for i := 1; i < len(pts); i++ {
		total += max(abs(pts[i][0]-pts[i-1][0]), abs(pts[i][1]-pts[i-1][1]))
	}
	return total
}

func main() {
	pts := [][2]int{{0, 0}, {1, 1}, {1, 2}}
	fmt.Println(minSteps(pts)) // 2
}
