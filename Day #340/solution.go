// Closest pair of points via divide & conquer. O(n log n) time, O(n) space.
// Sort by x, recurse on halves, merge by checking a y-sorted strip near the split.
package main

import (
	"fmt"
	"math"
	"sort"
)

type Pt struct{ x, y int }

func dist(a, b Pt) float64 {
	dx := float64(a.x - b.x)
	dy := float64(a.y - b.y)
	return math.Hypot(dx, dy)
}

func closest(px, py []Pt) (float64, Pt, Pt) {
	n := len(px)
	if n <= 3 {
		best := math.MaxFloat64
		var ba, bb Pt
		for i := 0; i < n; i++ {
			for j := i + 1; j < n; j++ {
				if d := dist(px[i], px[j]); d < best {
					best, ba, bb = d, px[i], px[j]
				}
			}
		}
		return best, ba, bb
	}
	mid := n / 2
	midx := px[mid].x
	lx, rx := px[:mid], px[mid:]
	leftCount := map[Pt]int{}
	for _, p := range lx {
		leftCount[p]++
	}
	var ly, ry []Pt
	for _, p := range py {
		if leftCount[p] > 0 {
			ly = append(ly, p)
			leftCount[p]--
		} else {
			ry = append(ry, p)
		}
	}
	dl, la, lb := closest(lx, ly)
	dr, ra, rb := closest(rx, ry)
	d, ba, bb := dl, la, lb
	if dr < dl {
		d, ba, bb = dr, ra, rb
	}
	var strip []Pt
	for _, p := range py {
		if math.Abs(float64(p.x-midx)) < d {
			strip = append(strip, p)
		}
	}
	for i := 0; i < len(strip); i++ {
		for j := i + 1; j < len(strip) && float64(strip[j].y-strip[i].y) < d; j++ {
			if dd := dist(strip[i], strip[j]); dd < d {
				d, ba, bb = dd, strip[i], strip[j]
			}
		}
	}
	return d, ba, bb
}

func main() {
	pts := []Pt{{1, 1}, {-1, -1}, {3, 4}, {6, 1}, {-1, -6}, {-4, -3}}
	px := make([]Pt, len(pts))
	py := make([]Pt, len(pts))
	copy(px, pts)
	copy(py, pts)
	sort.Slice(px, func(i, j int) bool {
		if px[i].x != px[j].x {
			return px[i].x < px[j].x
		}
		return px[i].y < px[j].y
	})
	sort.Slice(py, func(i, j int) bool {
		if py[i].y != py[j].y {
			return py[i].y < py[j].y
		}
		return py[i].x < py[j].x
	})
	_, a, b := closest(px, py)
	if a.x > b.x || (a.x == b.x && a.y > b.y) {
		a, b = b, a
	}
	fmt.Printf("[(%d, %d), (%d, %d)]\n", a.x, a.y, b.x, b.y)
}
