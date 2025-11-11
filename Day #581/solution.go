// Rectangle intersection area via overlap of x and y ranges. Time O(1), Space O(1).
package main

import "fmt"

func mini(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func maxi(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func intersectionArea(tlx1, tly1, w1, h1, tlx2, tly2, w2, h2 int) int {
	left1, right1, top1, bottom1 := tlx1, tlx1+w1, tly1, tly1-h1
	left2, right2, top2, bottom2 := tlx2, tlx2+w2, tly2, tly2-h2
	overlapW := mini(right1, right2) - maxi(left1, left2)
	overlapH := mini(top1, top2) - maxi(bottom1, bottom2)
	return maxi(0, overlapW) * maxi(0, overlapH)
}

func main() {
	fmt.Println(intersectionArea(1, 4, 3, 3, 0, 5, 4, 3))
}
