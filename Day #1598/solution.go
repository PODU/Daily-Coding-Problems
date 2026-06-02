// Round floats preserving sum: floor all, then round up the d elements with
// largest fractional parts (d = round(sum) - sum(floors)). Time O(n log n).
package main

import (
	"fmt"
	"math"
	"sort"
	"strconv"
	"strings"
)

func roundPreserve(x []float64) []int64 {
	n := len(x)
	y := make([]int64, n)
	var floorSum int64
	var sum float64
	for i, v := range x {
		f := int64(math.Floor(v))
		y[i] = f
		floorSum += f
		sum += v
	}
	target := int64(math.Round(sum))
	d := target - floorSum
	order := make([]int, n)
	for i := range order {
		order[i] = i
	}
	sort.Slice(order, func(a, b int) bool {
		fa := x[order[a]] - math.Floor(x[order[a]])
		fb := x[order[b]] - math.Floor(x[order[b]])
		return fa > fb
	})
	for i := int64(0); i < d; i++ {
		y[order[i]]++
	}
	return y
}

func main() {
	x := []float64{1.3, 2.3, 4.4}
	y := roundPreserve(x)

	parts := make([]string, len(y))
	for i, v := range y {
		parts[i] = strconv.FormatInt(v, 10)
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")

	var diff float64
	for i := range x {
		diff += math.Abs(x[i] - float64(y[i]))
	}
	fmt.Printf("abs diff = %.1f\n", diff)
}
