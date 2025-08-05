// Largest product of three: max(max1*max2*max3, min1*min2*max1) tracking top-3 & bottom-2. Time O(n), Space O(1).
package main

import (
	"fmt"
	"math"
)

func largestTripleProduct(nums []int) int64 {
	max1, max2, max3 := int64(math.MinInt64), int64(math.MinInt64), int64(math.MinInt64)
	min1, min2 := int64(math.MaxInt64), int64(math.MaxInt64)
	for _, v := range nums {
		x := int64(v)
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
	b := min1 * min2 * max1
	if a > b {
		return a
	}
	return b
}

func main() {
	nums := []int{-10, -10, 5, 2}
	fmt.Println(largestTripleProduct(nums))
}
