// 24 Game (fixed order): recursive split of contiguous list, combine with +,-,*,/ (float64).
// Time: O(1) for fixed 4 numbers. Space: O(1).
package main

import (
	"fmt"
	"math"
)

func solve(a []float64) []float64 {
	if len(a) == 1 {
		return []float64{a[0]}
	}
	var res []float64
	for i := 1; i < len(a); i++ {
		L := solve(a[:i])
		R := solve(a[i:])
		for _, l := range L {
			for _, r := range R {
				res = append(res, l+r, l-r, l*r)
				if math.Abs(r) > 1e-9 {
					res = append(res, l/r)
				}
			}
		}
	}
	return res
}

func can24(a []float64) bool {
	for _, v := range solve(a) {
		if math.Abs(v-24.0) < 1e-6 {
			return true
		}
	}
	return false
}

func main() {
	nums := []float64{5, 2, 7, 8}
	if can24(nums) {
		fmt.Println("True")
	} else {
		fmt.Println("False")
	}
}
