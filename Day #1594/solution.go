// Approach: Pairwise O(n^2) overlap check on axis-aligned rectangles (strict, positive-area).
// Time O(n^2), Space O(1).
package main

import "fmt"

type Rect struct{ x1, y1, x2, y2 float64 }

// top_left (x,y), dims (w,h): x in [x,x+w], y in [y-h,y]
func make_(x, y, w, h float64) Rect { return Rect{x, y - h, x + w, y} }

func overlap(a, b Rect) bool {
	return a.x1 < b.x2 && b.x1 < a.x2 && a.y1 < b.y2 && b.y1 < a.y2
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
	rs := []Rect{make_(1, 4, 3, 3), make_(-1, 3, 2, 1), make_(0, 5, 4, 3)}
	if anyOverlap(rs) {
		fmt.Println("true")
	} else {
		fmt.Println("false")
	}
}
