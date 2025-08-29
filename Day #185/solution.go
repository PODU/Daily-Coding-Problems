// Day 185: Area of intersection of two axis-aligned rectangles (top-left + width/height, y up).
// Overlap = max(0, dx) * max(0, dy). Time O(1), Space O(1).
package main

import "fmt"

type Rect struct{ left, top, w, h int }

func mn(a, b int) int { if a < b { return a }; return b }
func mx(a, b int) int { if a > b { return a }; return b }

func intersectionArea(a, b Rect) int {
	ox := mn(a.left+a.w, b.left+b.w) - mx(a.left, b.left)
	oy := mn(a.top, b.top) - mx(a.top-a.h, b.top-b.h)
	if ox <= 0 || oy <= 0 {
		return 0
	}
	return ox * oy
}

func main() {
	a := Rect{1, 4, 3, 3}
	b := Rect{0, 5, 4, 3}
	fmt.Println(intersectionArea(a, b))
}
