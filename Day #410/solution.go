// Day 410: Round floats so rounded sum is preserved with minimal total abs error.
// Floor all, then round up the R = round(sum)-sum(floors) elements with largest fractions. O(n log n) time, O(n) space.
package main

import (
	"fmt"
	"math"
	"sort"
	"strings"
)

func roundToSum(x []float64) []int64 {
	n := len(x)
	y := make([]int64, n)
	var floorSum int64
	var total float64
	for i, v := range x {
		y[i] = int64(math.Floor(v))
		floorSum += y[i]
		total += v
	}
	target := int64(math.Round(total))
	R := target - floorSum
	idx := make([]int, n)
	for i := range idx {
		idx[i] = i
	}
	sort.Slice(idx, func(a, b int) bool {
		fa := x[idx[a]] - math.Floor(x[idx[a]])
		fb := x[idx[b]] - math.Floor(x[idx[b]])
		return fa > fb
	})
	for i := int64(0); i < R && i < int64(n); i++ {
		y[idx[i]]++
	}
	return y
}

func main() {
	x := []float64{1.3, 2.3, 4.4}
	y := roundToSum(x)
	parts := make([]string, len(y))
	for i, v := range y {
		parts[i] = fmt.Sprintf("%d", v)
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
