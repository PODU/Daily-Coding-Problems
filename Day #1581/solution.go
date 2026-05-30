// Day 1581: Area of intersection of two axis-aligned rectangles.
// top_left=(x,y), dims=(w,h); x-range [x,x+w], y-range [y-h,y]. Overlap = clamped widths.
// Time: O(1); Space: O(1).
package main

import "fmt"

func max(a, b int) int { if a > b { return a }; return b }
func min(a, b int) int { if a < b { return a }; return b }

func intersectionArea(x1, y1, w1, h1, x2, y2, w2, h2 int) int {
	ow := min(x1+w1, x2+w2) - max(x1, x2)
	oh := min(y1, y2) - max(y1-h1, y2-h2)
	if ow <= 0 || oh <= 0 {
		return 0
	}
	return ow * oh
}

func main() {
	fmt.Println(intersectionArea(1, 4, 3, 3, 0, 5, 4, 3)) // 6
}
