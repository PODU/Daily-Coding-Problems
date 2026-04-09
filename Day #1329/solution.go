// Day 1329: Closest pair of points via divide & conquer. O(n log n) time.
// Sort by x, recurse on halves, then check the middle strip ordered by y (each point vs next ~7).
package main

import (
	"fmt"
	"math"
	"sort"
)

type P struct{ x, y int }

func dist(a, b P) float64 {
	dx := float64(a.x - b.x)
	dy := float64(a.y - b.y)
	return math.Sqrt(dx*dx + dy*dy)
}

func rec(px, py []P) (float64, P, P) {
	n := len(px)
	if n <= 3 {
		best := math.Inf(1)
		var a, b P
		for i := 0; i < n; i++ {
			for j := i + 1; j < n; j++ {
				if d := dist(px[i], px[j]); d < best {
					best, a, b = d, px[i], px[j]
				}
			}
		}
		return best, a, b
	}
	mid := n / 2
	pivot := px[mid]
	midX := pivot.x
	lx, rx := px[:mid], px[mid:]
	ly, ry := []P{}, []P{}
	for _, p := range py {
		if p.x < pivot.x || (p.x == pivot.x && p.y < pivot.y) {
			ly = append(ly, p)
		} else {
			ry = append(ry, p)
		}
	}
	bd, ba, bb := rec(lx, ly)
	rd, ra, rb := rec(rx, ry)
	if rd < bd {
		bd, ba, bb = rd, ra, rb
	}
	strip := []P{}
	for _, p := range py {
		if math.Abs(float64(p.x-midX)) < bd {
			strip = append(strip, p)
		}
	}
	for i := 0; i < len(strip); i++ {
		for j := i + 1; j < len(strip) && float64(strip[j].y-strip[i].y) < bd; j++ {
			if d := dist(strip[i], strip[j]); d < bd {
				bd, ba, bb = d, strip[i], strip[j]
			}
		}
	}
	return bd, ba, bb
}

func main() {
	pts := []P{{1, 1}, {-1, -1}, {3, 4}, {6, 1}, {-1, -6}, {-4, -3}}
	px := append([]P{}, pts...)
	py := append([]P{}, pts...)
	sort.Slice(px, func(i, j int) bool { return px[i].x < px[j].x || (px[i].x == px[j].x && px[i].y < px[j].y) })
	sort.Slice(py, func(i, j int) bool { return py[i].y < py[j].y })
	_, a, b := rec(px, py)
	if a.x > b.x || (a.x == b.x && a.y > b.y) {
		a, b = b, a
	}
	fmt.Printf("[(%d, %d), (%d, %d)]\n", a.x, a.y, b.x, b.y) // [(-1, -1), (1, 1)]
}
