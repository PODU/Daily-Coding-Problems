// Closest pair of points via divide-and-conquer on x, strip-merge on y.
// Each point tagged with an id for an unambiguous left/right split. Time: O(n log n). Space: O(n).
package main

import (
	"fmt"
	"sort"
)

type Pt struct {
	x, y int64
	id   int
}

func dist2(a, b Pt) float64 {
	dx := float64(a.x - b.x)
	dy := float64(a.y - b.y)
	return dx*dx + dy*dy
}

func rec(px, py []Pt) (Pt, Pt) {
	n := len(px)
	if n <= 3 {
		best := 1e18
		bp0, bp1 := px[0], px[0]
		for i := 0; i < n; i++ {
			for j := i + 1; j < n; j++ {
				if d := dist2(px[i], px[j]); d < best {
					best, bp0, bp1 = d, px[i], px[j]
				}
			}
		}
		return bp0, bp1
	}
	mid := n / 2
	midX := px[mid].x
	lx, rx := px[:mid], px[mid:]
	leftIds := map[int]bool{}
	for _, p := range lx {
		leftIds[p.id] = true
	}
	var ly, ry []Pt
	for _, p := range py {
		if leftIds[p.id] {
			ly = append(ly, p)
		} else {
			ry = append(ry, p)
		}
	}
	bl0, bl1 := rec(lx, ly)
	br0, br1 := rec(rx, ry)
	b0, b1 := bl0, bl1
	if dist2(br0, br1) < dist2(bl0, bl1) {
		b0, b1 = br0, br1
	}
	d2 := dist2(b0, b1)

	var strip []Pt
	for _, p := range py {
		dx := float64(p.x - midX)
		if dx*dx < d2 {
			strip = append(strip, p)
		}
	}
	for i := 0; i < len(strip); i++ {
		for j := i + 1; j < len(strip); j++ {
			dy := float64(strip[j].y - strip[i].y)
			if dy*dy >= d2 {
				break
			}
			if d := dist2(strip[i], strip[j]); d < d2 {
				d2, b0, b1 = d, strip[i], strip[j]
			}
		}
	}
	return b0, b1
}

func closestPair(points [][2]int64) (Pt, Pt) {
	pts := make([]Pt, len(points))
	for i, p := range points {
		pts[i] = Pt{p[0], p[1], i}
	}
	px := append([]Pt(nil), pts...)
	py := append([]Pt(nil), pts...)
	sort.Slice(px, func(i, j int) bool { return px[i].x < px[j].x || (px[i].x == px[j].x && px[i].y < px[j].y) })
	sort.Slice(py, func(i, j int) bool { return py[i].y < py[j].y || (py[i].y == py[j].y && py[i].x < py[j].x) })
	return rec(px, py)
}

func main() {
	points := [][2]int64{{1, 1}, {-1, -1}, {3, 4}, {6, 1}, {-1, -6}, {-4, -3}}
	a, b := closestPair(points)
	if a.x > b.x || (a.x == b.x && a.y > b.y) {
		a, b = b, a
	}
	fmt.Printf("[(%d, %d), (%d, %d)]\n", a.x, a.y, b.x, b.y)
	// [(-1, -1), (1, 1)]
}
