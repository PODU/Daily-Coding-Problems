// 24 game (fixed order): interval recursion combining left/right reachable
// values with + - * / using doubles + epsilon.
// Time: O(n^3 * S^2), Space: O(n^2 * S). Here n=4.
package main

import (
	"fmt"
	"math"
)

func solve(a []int, l, r int) []float64 {
	if l == r {
		return []float64{float64(a[l])}
	}
	var res []float64
	for m := l; m < r; m++ {
		L := solve(a, l, m)
		R := solve(a, m+1, r)
		for _, x := range L {
			for _, y := range R {
				res = append(res, x+y, x-y, x*y)
				if math.Abs(y) > 1e-9 {
					res = append(res, x/y)
				}
			}
		}
	}
	return res
}

func can24(a []int) bool {
	for _, v := range solve(a, 0, len(a)-1) {
		if math.Abs(v-24.0) < 1e-6 {
			return true
		}
	}
	return false
}

func main() {
	a := []int{5, 2, 7, 8}
	if can24(a) {
		fmt.Println("True")
	} else {
		fmt.Println("False")
	}
}
