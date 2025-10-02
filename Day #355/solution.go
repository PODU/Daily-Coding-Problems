// Round floats to ints keeping sum == round(total) with minimal total abs diff.
// Floor all; round up (T-F) elements with largest fractional parts (cheapest to push up). O(N log N) time, O(N) space.
package main

import (
	"fmt"
	"math"
	"sort"
	"strings"
)

func main() {
	x := []float64{1.3, 2.3, 4.4}
	n := len(x)
	y := make([]int64, n)
	var total float64
	var floorSum int64
	for i := 0; i < n; i++ {
		y[i] = int64(math.Floor(x[i]))
		floorSum += y[i]
		total += x[i]
	}
	target := int64(math.Round(total))
	need := target - floorSum

	idx := make([]int, n)
	for i := range idx {
		idx[i] = i
	}
	sort.Slice(idx, func(a, b int) bool {
		fa := x[idx[a]] - math.Floor(x[idx[a]])
		fb := x[idx[b]] - math.Floor(x[idx[b]])
		return fa > fb
	})

	for _, i := range idx {
		if need <= 0 {
			break
		}
		if x[i]-math.Floor(x[i]) > 0 {
			y[i]++
			need--
		}
	}

	parts := make([]string, n)
	for i, v := range y {
		parts[i] = fmt.Sprintf("%d", v)
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
