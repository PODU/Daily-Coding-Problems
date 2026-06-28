// Day 1729: Count buildings with a sunset (west) view.
// Single right-to-left pass tracking max height seen to the west; a building is
// visible iff strictly taller than all to its west. Time: O(n). Space: O(1).
package main

import "fmt"

func countSunsetViews(heights []int) int {
	count, maxWest := 0, 0
	// Scan from the west end (rightmost index) toward the east.
	for i := len(heights) - 1; i >= 0; i-- {
		if heights[i] > maxWest {
			count++
			maxWest = heights[i]
		}
	}
	return count
}

func main() {
	heights := []int{3, 7, 8, 3, 6, 1} // east -> west
	fmt.Println(countSunsetViews(heights)) // 1, 6, 8 visible => 3
}
