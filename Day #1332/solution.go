// Day 1332: Round each x_i up or down so sum(Y)=round(sum X) while minimizing sum|x_i-y_i|.
// Floor everything, then round up the k elements with the largest fractional parts (k=target-sum of floors). O(n log n).
package main

import (
	"fmt"
	"math"
	"sort"
)

func roundPreserveSum(x []float64) []int {
	n := len(x)
	s := 0.0
	for _, v := range x {
		s += v
	}
	target := int(math.Round(s))
	y := make([]int, n)
	frac := make([]float64, n)
	base := 0
	for i, v := range x {
		f := int(math.Floor(v))
		y[i] = f
		base += f
		frac[i] = v - float64(f)
	}
	k := target - base
	idx := make([]int, n)
	for i := range idx {
		idx[i] = i
	}
	sort.Slice(idx, func(a, b int) bool { return frac[idx[a]] > frac[idx[b]] })
	for i := 0; i < k; i++ {
		y[idx[i]]++
	}
	return y
}

func main() {
	fmt.Println(roundPreserveSum([]float64{1.3, 2.3, 4.4})) // [1 2 5]
}
