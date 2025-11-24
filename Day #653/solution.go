// Brute-force all O(k^2) pairs; rectangles overlap iff their projections strictly overlap on both axes.
// Sweep-line O(k log k) is possible but k^2 is clear. Time O(k^2), space O(k).
package main

import "fmt"

type Rect struct{ x1, y1, x2, y2 int }

// top_left (x,y), dims (w,h): x1=x, x2=x+w, y2=y(top), y1=y-h(bottom)
func make_(x, y, w, h int) Rect { return Rect{x, y - h, x + w, y} }

func overlap(a, b Rect) bool {
	return a.x1 < b.x2 && b.x1 < a.x2 && a.y1 < b.y2 && b.y1 < a.y2
}

func main() {
	rects := []Rect{
		make_(1, 4, 3, 3),   // R1
		make_(-1, 3, 2, 1),  // R2
		make_(0, 5, 4, 3),   // R3
	}
	any := false
	for i := 0; i < len(rects) && !any; i++ {
		for j := i + 1; j < len(rects); j++ {
			if overlap(rects[i], rects[j]) {
				any = true
				break
			}
		}
	}
	if any {
		fmt.Println("true")
	} else {
		fmt.Println("false")
	}
}
