// Day 711: Point strictly inside polygon. First reject boundary via on-segment
// test, then ray-casting parity test. Time O(N) per query.
package main

import (
	"fmt"
	"math"
)

type P struct{ x, y float64 }

func onSeg(a, b, p P) bool {
	cross := (b.x-a.x)*(p.y-a.y) - (b.y-a.y)*(p.x-a.x)
	if math.Abs(cross) > 1e-9 {
		return false
	}
	return math.Min(a.x, b.x)-1e-9 <= p.x && p.x <= math.Max(a.x, b.x)+1e-9 &&
		math.Min(a.y, b.y)-1e-9 <= p.y && p.y <= math.Max(a.y, b.y)+1e-9
}

func inside(poly []P, p P) bool {
	n := len(poly)
	for i := 0; i < n; i++ {
		if onSeg(poly[i], poly[(i+1)%n], p) {
			return false
		}
	}
	in := false
	j := n - 1
	for i := 0; i < n; i++ {
		if (poly[i].y > p.y) != (poly[j].y > p.y) {
			xint := (poly[j].x-poly[i].x)*(p.y-poly[i].y)/(poly[j].y-poly[i].y) + poly[i].x
			if p.x < xint {
				in = !in
			}
		}
		j = i
	}
	return in
}

func b2s(b bool) string {
	if b {
		return "True"
	}
	return "False"
}

func main() {
	sq := []P{{0, 0}, {4, 0}, {4, 4}, {0, 4}}
	fmt.Println(b2s(inside(sq, P{2, 2})))
	fmt.Println(b2s(inside(sq, P{4, 2})))
	fmt.Println(b2s(inside(sq, P{5, 5})))
}
