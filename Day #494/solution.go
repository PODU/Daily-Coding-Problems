// Day 494: Maximum circular subarray sum (empty allowed => 0).
// max(maxKadane, total - minKadane); if all negative answer is 0. Time O(n), Space O(1).
package main

import (
	"fmt"
	"math"
)

func maxCircularSubarray(a []int64) int64 {
	var total int64 = 0
	maxK := int64(math.MinInt64)
	var curMax int64 = 0
	minK := int64(math.MaxInt64)
	var curMin int64 = 0
	for _, x := range a {
		total += x
		if curMax+x > x {
			curMax = curMax + x
		} else {
			curMax = x
		}
		if curMax > maxK {
			maxK = curMax
		}
		if curMin+x < x {
			curMin = curMin + x
		} else {
			curMin = x
		}
		if curMin < minK {
			minK = curMin
		}
	}
	if maxK < 0 {
		return 0 // all negative -> empty subarray
	}
	if total-minK > maxK {
		return total - minK
	}
	return maxK
}

func main() {
	fmt.Println(maxCircularSubarray([]int64{8, -1, 3, 4})) // 15
	fmt.Println(maxCircularSubarray([]int64{-4, 5, 1, 0})) // 6
}
