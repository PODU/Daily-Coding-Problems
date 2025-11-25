// Power set via bitmasks; sort subsets by size then numeric order. Time O(n*2^n), Space O(2^n).
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
	sort.SliceStable(subsets, func(a, b int) bool {
		if len(subsets[a]) != len(subsets[b]) {
			return len(subsets[a]) < len(subsets[b])
		}
		for i := 0; i < len(subsets[a]); i++ {
			if subsets[a][i] != subsets[b][i] {
				return subsets[a][i] < subsets[b][i]
			}
		}
		return false
	})
	var parts []string
	for _, sub := range subsets {
		var elems []string
		for _, x := range sub {
			elems = append(elems, strconv.Itoa(x))
		}
		parts = append(parts, "{"+strings.Join(elems, ", ")+"}")
	}
	fmt.Println("{" + strings.Join(parts, ", ") + "}")
}
