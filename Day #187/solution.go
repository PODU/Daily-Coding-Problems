// Day 187: Do any two rectangles overlap (containment counts; edge-touching does not).
// Pairwise interior-overlap test. Time O(n^2), Space O(1). (Sweep line gives O(n log n).)
package main

import "fmt"

type Rect struct{ left, top, w, h int }

func mn(a, b int) int { if a < b { return a }; return b }
func mx(a, b int) int { if a > b { return a }; return b }

func overlap(a, b Rect) bool {
	ox := mn(a.left+a.w, b.left+b.w) - mx(a.left, b.left)
	oy := mn(a.top, b.top) - mx(a.top-a.h, b.top-b.h)
	return ox > 0 && oy > 0
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
	rs := []Rect{{1, 4, 3, 3}, {-1, 3, 2, 1}, {0, 5, 4, 3}}
	fmt.Println(anyOverlap(rs))
}
