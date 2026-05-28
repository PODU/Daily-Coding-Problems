// Day 1579: Maximum circular subarray sum (empty allowed -> 0).
// ans = max(Kadane-with-empty, total - minSubarray). Time: O(n); Space: O(1).
package main

import (
	"fmt"
	"math"
)

func maxCircular(a []int) int {
	total, maxEnd, maxSum := 0, 0, 0
	minEnd, minSum := 0, math.MaxInt64
	for _, x := range a {
		total += x
		if maxEnd+x > x {
			maxEnd += x
		} else {
			maxEnd = x
		}
		if maxEnd > maxSum {
			maxSum = maxEnd
		}
		if minEnd+x < x {
			minEnd += x
		} else {
			minEnd = x
		}
		if minEnd < minSum {
			minSum = minEnd
		}
	}
	wrap := total - minSum
	if wrap > maxSum {
		return wrap
	}
	return maxSum
}

func main() {
	fmt.Println(maxCircular([]int{8, -1, 3, 4})) // 15
	fmt.Println(maxCircular([]int{-4, 5, 1, 0})) // 6
}
