// Day 600: Closest pair of points on a plane.
// Approach: divide & conquer with a y-sorted strip check. Time O(n log n), Space O(n).
package main

import (
	"fmt"
	"math"
	"sort"
)

type P struct{ x, y int64 }

func dist2(a, b P) float64 {
	dx := float64(a.x - b.x)
	dy := float64(a.y - b.y)
	return dx*dx + dy*dy
}

func rec(px []P, lo, hi int) (float64, P, P) {
	n := hi - lo
	best := math.Inf(1)
	var ba, bb P
	if n <= 3 {
		for i := lo; i < hi; i++ {
			for j := i + 1; j < hi; j++ {
				if d := dist2(px[i], px[j]); d < best {
					best, ba, bb = d, px[i], px[j]
				}
			}
		}
		return best, ba, bb
	}
	mid := (lo + hi) / 2
	midx := px[mid].x
	bl, la, lb := rec(px, lo, mid)
	br, ra, rb := rec(px, mid, hi)
	if bl <= br {
		best, ba, bb = bl, la, lb
	} else {
		best, ba, bb = br, ra, rb
	}
	dd := math.Sqrt(best)
	var strip []P
	for i := lo; i < hi; i++ {
		if math.Abs(float64(px[i].x-midx)) < dd {
			strip = append(strip, px[i])
		}
	}
	sort.Slice(strip, func(i, j int) bool { return strip[i].y < strip[j].y })
	for i := 0; i < len(strip); i++ {
		for j := i + 1; j < len(strip) && float64(strip[j].y-strip[i].y) < dd; j++ {
			if d := dist2(strip[i], strip[j]); d < best {
				best, ba, bb = d, strip[i], strip[j]
				dd = math.Sqrt(best)
			}
		}
	}
	return best, ba, bb
}

func main() {
	pts := []P{{1, 1}, {-1, -1}, {3, 4}, {6, 1}, {-1, -6}, {-4, -3}}
	sort.Slice(pts, func(i, j int) bool {
		if pts[i].x != pts[j].x {
			return pts[i].x < pts[j].x
		}
		return pts[i].y < pts[j].y
	})
	_, a, b := rec(pts, 0, len(pts))
	if a.x > b.x || (a.x == b.x && a.y > b.y) {
		a, b = b, a
	}
	fmt.Printf("[(%d, %d), (%d, %d)]\n", a.x, a.y, b.x, b.y)
}
