// Day 1290: Strict point-in-polygon test (ray casting), boundary counts as outside.
// Check edges for on-boundary, then parity of rightward ray crossings. Time O(n), Space O(1).
package main

import (
	"fmt"
	"math"
)

type Pt struct{ x, y float64 }

func onSegment(a, b, p Pt) bool {
	cross := (b.x-a.x)*(p.y-a.y) - (b.y-a.y)*(p.x-a.x)
	if math.Abs(cross) > 1e-9 {
		return false
	}
	return p.x >= math.Min(a.x, b.x)-1e-9 && p.x <= math.Max(a.x, b.x)+1e-9 &&
		p.y >= math.Min(a.y, b.y)-1e-9 && p.y <= math.Max(a.y, b.y)+1e-9
}

func inside(poly []Pt, p Pt) bool {
	n := len(poly)
	for i := 0; i < n; i++ {
		if onSegment(poly[i], poly[(i+1)%n], p) {
			return false
		}
	}
	res := false
	j := n - 1
	for i := 0; i < n; i++ {
		xi, yi := poly[i].x, poly[i].y
		xj, yj := poly[j].x, poly[j].y
		if (yi > p.y) != (yj > p.y) {
			xint := (xj-xi)*(p.y-yi)/(yj-yi) + xi
			if p.x < xint {
				res = !res
			}
		}
		j = i
	}
	return res
}

func main() {
	square := []Pt{{0, 0}, {4, 0}, {4, 4}, {0, 4}}
	fmt.Println(inside(square, Pt{2, 2})) // true
	fmt.Println(inside(square, Pt{5, 5})) // false
	fmt.Println(inside(square, Pt{4, 2})) // false (boundary)
}
