// Day 1331: Does any pair of rectangles overlap (full containment counts; edge-touching does not)?
// Convert top_left+dimensions to [xmin,xmax,ymin,ymax]; pairwise strict-interval overlap test. O(n^2).
package main

import "fmt"

type Rect struct{ xmin, xmax, ymin, ymax int }

func makeRect(tlx, tly, w, h int) Rect {
	return Rect{tlx, tlx + w, tly - h, tly}
}

func overlap(a, b Rect) bool {
	return a.xmin < b.xmax && b.xmin < a.xmax && a.ymin < b.ymax && b.ymin < a.ymax
}

func main() {
	rs := []Rect{
		makeRect(1, 4, 3, 3),
		makeRect(-1, 3, 2, 1),
		makeRect(0, 5, 4, 3),
	}
	any := false
	for i := 0; i < len(rs) && !any; i++ {
		for j := i + 1; j < len(rs); j++ {
			if overlap(rs[i], rs[j]) {
				any = true
				break
			}
		}
	}
	fmt.Println(any) // true
}
