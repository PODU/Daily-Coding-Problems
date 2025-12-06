// Day 706: 24 Game (fixed order). Try every parenthesization over the fixed
// sequence, combining sub-results with +,-,*,/. Time ~O(1) for 4 numbers.
package main

import (
	"fmt"
	"math"
)

func solve(nums []float64) []float64 {
	if len(nums) == 1 {
		return []float64{nums[0]}
	}
	var res []float64
	for i := 1; i < len(nums); i++ {
		lv := solve(nums[:i])
		rv := solve(nums[i:])
		for _, a := range lv {
			for _, b := range rv {
				res = append(res, a+b, a-b, a*b)
				if math.Abs(b) > 1e-9 {
					res = append(res, a/b)
				}
			}
		}
	}
	return res
}

func game24(digits []int) bool {
	nums := make([]float64, len(digits))
	for i, d := range digits {
		nums[i] = float64(d)
	}
	for _, v := range solve(nums) {
		if math.Abs(v-24.0) < 1e-6 {
			return true
		}
	}
	return false
}

func main() {
	if game24([]int{5, 2, 7, 8}) {
		fmt.Println("True")
	} else {
		fmt.Println("False")
	}
}
