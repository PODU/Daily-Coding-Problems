// Largest product of three: one pass tracking 3 largest + 2 smallest.
// answer = max(max1*max2*max3, max1*min1*min2). Time O(n), Space O(1).
package main

import (
	"fmt"
	"math"
)

func largestProduct(nums []int64) int64 {
	max1, max2, max3 := int64(math.MinInt64), int64(math.MinInt64), int64(math.MinInt64)
	min1, min2 := int64(math.MaxInt64), int64(math.MaxInt64)
	for _, x := range nums {
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
	a := max1 * max2 * max3
	b := max1 * min1 * min2
	if a > b {
		return a
	}
	return b
}

func main() {
	nums := []int64{-10, -10, 5, 2}
	fmt.Println(largestProduct(nums))
}
