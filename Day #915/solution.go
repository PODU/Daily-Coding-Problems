// Floor all; round up the `deficit` elements with largest fractional parts to match round(sum). O(n log n) time, O(n) space.
package main

import (
	"fmt"
	"math"
	"sort"
	"strings"
)

func roundToSum(x []float64) []int {
	n := len(x)
	y := make([]int, n)
	floorSum := 0
	total := 0.0
	for i, v := range x {
		f := math.Floor(v)
		y[i] = int(f)
		floorSum += int(f)
		total += v
	}
	target := int(math.Round(total))
	deficit := target - floorSum
	idx := make([]int, n)
	for i := range idx {
		idx[i] = i
	}
	sort.Slice(idx, func(a, b int) bool {
		fa := x[idx[a]] - math.Floor(x[idx[a]])
		fb := x[idx[b]] - math.Floor(x[idx[b]])
		return fa > fb
	})
	for k := 0; k < deficit && k < n; k++ {
		y[idx[k]]++
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
