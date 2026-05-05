// Day 1472: Largest product of any three integers.
// Track 3 largest and 2 smallest in one pass; max of two candidate products.
// Time O(N), Space O(1).
package main

import (
	"fmt"
	"math"
)

func maxProductThree(nums []int64) int64 {
	max1, max2, max3 := int64(math.MinInt64), int64(math.MinInt64), int64(math.MinInt64)
	min1, min2 := int64(math.MaxInt64), int64(math.MaxInt64)
	for _, n := range nums {
		if n > max1 {
			max3, max2, max1 = max2, max1, n
		} else if n > max2 {
			max3, max2 = max2, n
		} else if n > max3 {
			max3 = n
		}
		if n < min1 {
			min2, min1 = min1, n
		} else if n < min2 {
			min2 = n
		}
	}
	a := max1 * max2 * max3
	b := max1 * min1 * min2
	if a > b {
		return a
	}
	return b
}

func main() {
	fmt.Println(maxProductThree([]int64{-10, -10, 5, 2})) // 500
}
