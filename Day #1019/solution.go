// Rectangle intersection area: O(1) time, O(1) space.
// top_left is top y, height extends downward. x_overlap*y_overlap clamped at 0.
package main

import (
	"fmt"
	"math"
)

type Rect struct{ left, top, width, height float64 }

func intersectArea(a, b Rect) float64 {
	aRight, bRight := a.left+a.width, b.left+b.width
	aBottom, bBottom := a.top-a.height, b.top-b.height
	xo := math.Max(0, math.Min(aRight, bRight)-math.Max(a.left, b.left))
	yo := math.Max(0, math.Min(a.top, b.top)-math.Max(aBottom, bBottom))
	return xo * yo
}

func main() {
	r1 := Rect{1, 4, 3, 3}
	r2 := Rect{0, 5, 4, 3}
	fmt.Printf("%g\n", intersectArea(r1, r2))
}
