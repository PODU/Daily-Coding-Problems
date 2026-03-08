// Day 1175: Maximum subarray sum in a circular array (empty allowed -> 0).
// Answer = max(0, kadaneMax, total - kadaneMin); total-min covers the wrap case.
// Time O(N), Space O(1).
package main

import "fmt"

func maxCircularSubarray(a []int) int {
	total, curMax, bestMax, curMin, bestMin := 0, 0, 0, 0, 0
	max := func(x, y int) int { if x > y { return x }; return y }
	min := func(x, y int) int { if x < y { return x }; return y }
	for _, x := range a {
		total += x
		curMax = max(x, curMax+x)
		bestMax = max(bestMax, curMax)
		curMin = min(x, curMin+x)
		bestMin = min(bestMin, curMin)
	}
	return max(0, max(bestMax, total-bestMin))
}

func main() {
	fmt.Println(maxCircularSubarray([]int{8, -1, 3, 4})) // 15
	fmt.Println(maxCircularSubarray([]int{-4, 5, 1, 0})) // 6
}
