// Largest product of three: track top-3 max and bottom-2 min in one pass.
// Answer = max(max1*max2*max3, min1*min2*max1). O(n) time, O(1) space.
package main

import (
	"fmt"
	"math"
)

func largestProductOfThree(a []int) int64 {
	var max1, max2, max3 int64 = math.MinInt64, math.MinInt64, math.MinInt64
	var min1, min2 int64 = math.MaxInt64, math.MaxInt64
	for _, xi := range a {
		x := int64(xi)
		if x > max1 {
			max3, max2, max1 = max2, max1, x
		} else if x > max2 {
			max3, max2 = max2, x
		} else if x > max3 {
			max3 = x
		}
		if x < min1 {
			min2, min1 = min1, x
		} else if x < min2 {
			min2 = x
		}
	}
	p1 := max1 * max2 * max3
	p2 := min1 * min2 * max1
	if p1 > p2 {
		return p1
	}
	return p2
}

func main() {
	fmt.Println(largestProductOfThree([]int{-10, -10, 5, 2})) // 500
}
