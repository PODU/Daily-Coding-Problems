// Day 852: maximum circular subarray sum (empty allowed -> 0).
// answer = max(maxKadane clamped at 0, total - minKadane). O(n) time, O(1) space.
package main

import (
	"fmt"
	"math"
)

func maxCircular(a []int64) int64 {
	var total int64 = 0
	maxK := int64(math.MinInt64)
	minK := int64(math.MaxInt64)
	var curMax, curMin int64 = 0, 0
	for _, x := range a {
		total += x
		if curMax+x > x {
			curMax += x
		} else {
			curMax = x
		}
		if curMax > maxK {
			maxK = curMax
		}
		if curMin+x < x {
			curMin += x
		} else {
			curMin = x
		}
		if curMin < minK {
			minK = curMin
		}
	}
	nonWrap := maxK
	if nonWrap < 0 {
		nonWrap = 0
	}
	wrap := total - minK
	if wrap > nonWrap {
		return wrap
	}
	return nonWrap
}

func main() {
	fmt.Println(maxCircular([]int64{8, -1, 3, 4})) // 15
	fmt.Println(maxCircular([]int64{-4, 5, 1, 0})) // 6
}
