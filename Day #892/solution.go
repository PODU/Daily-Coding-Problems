// Power set via bitmask iteration over 2^n subsets, then sorted by (size, elements).
// Time: O(n*2^n), Space: O(n*2^n) to hold all subsets.
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
		var cur []int
		for i := 0; i < n; i++ {
			if mask&(1<<i) != 0 {
				cur = append(cur, s[i])
			}
		}
		subsets = append(subsets, cur)
	}
	sort.Slice(subsets, func(a, b int) bool {
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
	parts := make([]string, len(subsets))
	for i, sub := range subsets {
		elems := make([]string, len(sub))
		for j, v := range sub {
			elems[j] = strconv.Itoa(v)
		}
		parts[i] = "{" + strings.Join(elems, ", ") + "}"
	}
	fmt.Println("{" + strings.Join(parts, ", ") + "}")
}
