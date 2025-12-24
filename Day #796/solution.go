// Day 796: Point strictly inside a polygon.
// Ray-casting (even-odd rule) + on-boundary check. Time O(N), Space O(1).
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
	return math.Min(a.x, b.x)-1e-9 <= p.x && p.x <= math.Max(a.x, b.x)+1e-9 &&
		math.Min(a.y, b.y)-1e-9 <= p.y && p.y <= math.Max(a.y, b.y)+1e-9
}

func insidePolygon(poly []Pt, p Pt) bool {
	n := len(poly)
	for i := 0; i < n; i++ {
		if onSegment(poly[i], poly[(i+1)%n], p) {
			return false
		}
	}
	inside := false
	for i, j := 0, n-1; i < n; i++ {
		if (poly[i].y > p.y) != (poly[j].y > p.y) &&
			p.x < (poly[j].x-poly[i].x)*(p.y-poly[i].y)/(poly[j].y-poly[i].y)+poly[i].x {
			inside = !inside
		}
		j = i
	}
	return inside
}

func main() {
	square := []Pt{{0, 0}, {4, 0}, {4, 4}, {0, 4}}
	fmt.Println(insidePolygon(square, Pt{2, 2})) // true
	fmt.Println(insidePolygon(square, Pt{4, 2})) // false (boundary)
	fmt.Println(insidePolygon(square, Pt{5, 5})) // false
}
