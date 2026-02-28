// Power set via bitmask enumeration, sorted by (size, lexicographic). O(2^n * n).
package main

import (
	"fmt"
	"sort"
	"strconv"
	"strings"
)

func main() {
	nums := []int{1, 2, 3}
	n := len(nums)
	var subsets [][]int
	for mask := 0; mask < (1 << n); mask++ {
		var s []int
		for i := 0; i < n; i++ {
			if mask&(1<<i) != 0 {
				s = append(s, nums[i])
			}
		}
		subsets = append(subsets, s)
	}
	sort.Slice(subsets, func(i, j int) bool {
		a, b := subsets[i], subsets[j]
		if len(a) != len(b) {
			return len(a) < len(b)
		}
		for k := 0; k < len(a); k++ {
			if a[k] != b[k] {
				return a[k] < b[k]
			}
		}
		return false
	})
	parts := make([]string, 0, len(subsets))
	for _, s := range subsets {
		elems := make([]string, 0, len(s))
		for _, v := range s {
			elems = append(elems, strconv.Itoa(v))
		}
		parts = append(parts, "["+strings.Join(elems, ", ")+"]")
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
