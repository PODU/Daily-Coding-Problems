// Closest pair of points via divide & conquer: sort by x, recurse, merge with strip check by y.
// O(n log n) time, O(n) space.
package main

import (
	"fmt"
	"math"
	"sort"
)

type Pt struct{ x, y int64 }

var bestD = math.MaxFloat64
var bestA, bestB Pt

func dist(a, b Pt) float64 {
	dx := float64(a.x - b.x)
	dy := float64(a.y - b.y)
	return math.Sqrt(dx*dx + dy*dy)
}

func consider(a, b Pt) {
	d := dist(a, b)
	if d < bestD {
		bestD, bestA, bestB = d, a, b
	}
}

// px sorted by x in [lo,hi); returns slice sorted by y
func rec(px []Pt, lo, hi int) []Pt {
	n := hi - lo
	if n <= 3 {
		pts := make([]Pt, n)
		copy(pts, px[lo:hi])
		for i := 0; i < len(pts); i++ {
			for j := i + 1; j < len(pts); j++ {
				consider(pts[i], pts[j])
			}
		}
		sort.Slice(pts, func(i, j int) bool { return pts[i].y < pts[j].y })
		return pts
	}
	mid := lo + n/2
	midx := px[mid].x
	left := rec(px, lo, mid)
	right := rec(px, mid, hi)
	merged := make([]Pt, 0, n)
	i, j := 0, 0
	for i < len(left) && j < len(right) {
		if left[i].y <= right[j].y {
			merged = append(merged, left[i])
			i++
		} else {
			merged = append(merged, right[j])
			j++
		}
	}
	merged = append(merged, left[i:]...)
	merged = append(merged, right[j:]...)
	strip := []Pt{}
	for _, p := range merged {
		d := p.x - midx
		if d < 0 {
			d = -d
		}
		if float64(d) < bestD {
			strip = append(strip, p)
		}
	}
	for a := 0; a < len(strip); a++ {
		for b := a + 1; b < len(strip) && float64(strip[b].y-strip[a].y) < bestD; b++ {
			consider(strip[a], strip[b])
		}
	}
	return merged
}

func main() {
	pts := []Pt{{1, 1}, {-1, -1}, {3, 4}, {6, 1}, {-1, -6}, {-4, -3}}
	sort.Slice(pts, func(i, j int) bool {
		if pts[i].x != pts[j].x {
			return pts[i].x < pts[j].x
		}
		return pts[i].y < pts[j].y
	})
	rec(pts, 0, len(pts))
	a, b := bestA, bestB
	if a.x > b.x || (a.x == b.x && a.y > b.y) {
		a, b = b, a
	}
	fmt.Printf("[(%d, %d), (%d, %d)]\n", a.x, a.y, b.x, b.y)
}
