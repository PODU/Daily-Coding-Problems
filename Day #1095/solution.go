// Detect if any pair of axis-aligned rectangles overlap (containment counts). Time O(n^2), Space O(n).
package main

import "fmt"

type Rect struct{ minx, maxx, miny, maxy float64 }

func fromTopLeft(x, y, w, h float64) Rect {
	return Rect{x, x + w, y - h, y} // top_left, width grows right, height grows down
}

func overlap(a, b Rect) bool {
	return a.minx < b.maxx && b.minx < a.maxx && a.miny < b.maxy && b.miny < a.maxy
}

func anyOverlap(rs []Rect) bool {
	for i := 0; i < len(rs); i++ {
		for j := i + 1; j < len(rs); j++ {
			if overlap(rs[i], rs[j]) {
				return true
			}
		}
	}
	return false
}

func main() {
	rs := []Rect{
		fromTopLeft(1, 4, 3, 3),
		fromTopLeft(-1, 3, 2, 1),
		fromTopLeft(0, 5, 4, 3)}
	fmt.Println(anyOverlap(rs))
}
