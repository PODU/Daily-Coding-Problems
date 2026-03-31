// Day 1281: Area of intersection of two axis-aligned rectangles.
// Overlap on each axis = min(rights)-max(lefts), clamped at 0. Time O(1), Space O(1).
package main

import "fmt"

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}
func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

// Each rectangle: top-left (x, y) with width w and height h (y grows upward).
func intersectArea(x1, y1, w1, h1, x2, y2, w2, h2 int) int {
	ow := min(x1+w1, x2+w2) - max(x1, x2)
	oh := min(y1, y2) - max(y1-h1, y2-h2)
	if ow <= 0 || oh <= 0 {
		return 0
	}
	return ow * oh
}

func main() {
	fmt.Println(intersectArea(1, 4, 3, 3, 0, 5, 4, 3)) // 6
}
