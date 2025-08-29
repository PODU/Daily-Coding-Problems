// Day 190: Maximum circular subarray sum, empty subarray (sum 0) allowed.
// ans = max(0, maxKadane, total - minKadane). Time O(n), Space O(1).
package main

import (
	"fmt"
	"math"
)

func maxCircularSum(a []int) int {
	total := 0
	maxK, curMax := math.MinInt64, 0
	minK, curMin := math.MaxInt64, 0
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
	ans := maxK
	if total-minK > ans {
		ans = total - minK
	}
	if ans < 0 {
		ans = 0
	}
	return ans
}

func main() {
	fmt.Println(maxCircularSum([]int{8, -1, 3, 4}))
	fmt.Println(maxCircularSum([]int{-4, 5, 1, 0}))
}
