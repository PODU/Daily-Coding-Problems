// 24 game, fixed order: recursively split the sequence at each position, combine left/right results with +,-,*,/ (eps for div). O(4^n) over splits; O(n) depth.
package main

import (
	"fmt"
	"math"
)

const eps = 1e-6

func solve(nums []float64, lo, hi int) []float64 {
	if hi-lo == 1 {
		return []float64{nums[lo]}
	}
	var res []float64
	for mid := lo + 1; mid < hi; mid++ {
		L := solve(nums, lo, mid)
		R := solve(nums, mid, hi)
		for _, a := range L {
			for _, b := range R {
				res = append(res, a+b, a-b, a*b)
				if math.Abs(b) > eps {
					res = append(res, a/b)
				}
			}
		}
	}
	return res
}

func canReach24(in []int) bool {
	nums := make([]float64, len(in))
	for i, v := range in {
		nums[i] = float64(v)
	}
	for _, v := range solve(nums, 0, len(nums)) {
		if math.Abs(v-24.0) < eps {
			return true
		}
	}
	return false
}

func main() {
	input := []int{5, 2, 7, 8}
	if canReach24(input) {
		fmt.Println("True")
	} else {
		fmt.Println("False")
	}
}
