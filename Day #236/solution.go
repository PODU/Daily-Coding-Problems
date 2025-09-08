// Point in polygon: ray-casting (even-odd rule). Boundary points are detected separately
// and return false. Time: O(N), Space: O(1).
package main

import (
	"fmt"
	"math"
)

func onSegment(px, py, ax, ay, bx, by float64) bool {
	cross := (bx-ax)*(py-ay) - (by-ay)*(px-ax)
	if math.Abs(cross) > 1e-9 {
		return false
	}
	return math.Min(ax, bx)-1e-9 <= px && px <= math.Max(ax, bx)+1e-9 &&
		math.Min(ay, by)-1e-9 <= py && py <= math.Max(ay, by)+1e-9
}

func inside(poly [][2]float64, px, py float64) bool {
	n := len(poly)
	res := false
	j := n - 1
	for i := 0; i < n; i++ {
		xi, yi := poly[i][0], poly[i][1]
		xj, yj := poly[j][0], poly[j][1]
		if onSegment(px, py, xi, yi, xj, yj) {
			return false // boundary
		}
		if (yi > py) != (yj > py) && px < (xj-xi)*(py-yi)/(yj-yi)+xi {
			res = !res
		}
		j = i
	}
	return res
}

func main() {
	poly := [][2]float64{{0, 0}, {4, 0}, {4, 4}, {0, 4}}
	fmt.Println(inside(poly, 2, 2)) // true
	fmt.Println(inside(poly, 4, 2)) // false (boundary)
	fmt.Println(inside(poly, 5, 5)) // false (outside)
}
