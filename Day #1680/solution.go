// Day 1680: Strict point-in-polygon. Reject boundary via on-segment test, else
// ray-casting parity. Time O(N), Space O(1).
package main

import (
	"fmt"
	"math"
)

func onSeg(x1, y1, x2, y2, px, py float64) bool {
	cross := (x2-x1)*(py-y1) - (y2-y1)*(px-x1)
	if math.Abs(cross) > 1e-9 {
		return false
	}
	return px >= math.Min(x1, x2)-1e-9 && px <= math.Max(x1, x2)+1e-9 &&
		py >= math.Min(y1, y2)-1e-9 && py <= math.Max(y1, y2)+1e-9
}

func inside(poly [][2]float64, px, py float64) bool {
	n := len(poly)
	for i := 0; i < n; i++ {
		a, b := poly[i], poly[(i+1)%n]
		if onSeg(a[0], a[1], b[0], b[1], px, py) {
			return false
		}
	}
	res := false
	j := n - 1
	for i := 0; i < n; i++ {
		xi, yi := poly[i][0], poly[i][1]
		xj, yj := poly[j][0], poly[j][1]
		if (yi > py) != (yj > py) && px < (xj-xi)*(py-yi)/(yj-yi)+xi {
			res = !res
		}
		j = i
	}
	return res
}

func main() {
	sq := [][2]float64{{0, 0}, {4, 0}, {4, 4}, {0, 4}}
	fmt.Println(inside(sq, 2, 2)) // true
	fmt.Println(inside(sq, 4, 2)) // false
	fmt.Println(inside(sq, 5, 5)) // false
}
