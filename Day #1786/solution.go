// Recursively combine adjacent pairs (preserves order, covers all parenthesizations)
// applying +,-,*,/ until one value remains; check |v-24|<1e-6.
// Time O(exponential in n), Space O(n) recursion. Here n=4.
package main

import (
	"fmt"
	"math"
)

func solve(nums []float64) bool {
	if len(nums) == 1 {
		return math.Abs(nums[0]-24.0) < 1e-6
	}
	for i := 0; i+1 < len(nums); i++ {
		a, b := nums[i], nums[i+1]
		results := []float64{a + b, a - b, a * b}
		if math.Abs(b) > 1e-9 {
			results = append(results, a/b)
		}
		for _, r := range results {
			next := make([]float64, 0, len(nums)-1)
			next = append(next, nums[:i]...)
			next = append(next, r)
			next = append(next, nums[i+2:]...)
			if solve(next) {
				return true
			}
		}
	}
	return false
}

func canGet24(nums []int) bool {
	d := make([]float64, len(nums))
	for i, n := range nums {
		d[i] = float64(n)
	}
	return solve(d)
}

func main() {
	if canGet24([]int{5, 2, 7, 8}) {
		fmt.Println("True")
	} else {
		fmt.Println("False")
	}
}
