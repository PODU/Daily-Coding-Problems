// Power set via bitmask 0..2^n-1, then order subsets by (size, element order).
// O(2^n * n) time, O(2^n * n) space.
package main

import (
	"fmt"
	"sort"
	"strconv"
	"strings"
)

func main() {
	s := []int{1, 2, 3}
	n := len(s)
	var subsets [][]int
	for mask := 0; mask < (1 << n); mask++ {
		var sub []int
		for i := 0; i < n; i++ {
			if mask&(1<<i) != 0 {
				sub = append(sub, s[i])
			}
		}
		subsets = append(subsets, sub)
	}
	sort.Slice(subsets, func(i, j int) bool {
		a, b := subsets[i], subsets[j]
		if len(a) != len(b) {
			return len(a) < len(b)
		}
		for k := range a {
			if a[k] != b[k] {
				return a[k] < b[k]
			}
		}
		return false
	})
	parts := make([]string, len(subsets))
	for i, sub := range subsets {
		elems := make([]string, len(sub))
		for j, x := range sub {
			elems[j] = strconv.Itoa(x)
		}
		parts[i] = "{" + strings.Join(elems, ", ") + "}"
	}
	fmt.Println("{" + strings.Join(parts, ", ") + "}")
}
