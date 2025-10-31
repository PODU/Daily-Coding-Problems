// k==1: smallest rotation (try all n rotations). k>=2: sorted string (any pair
// of front letters can be reordered). Time O(n^2) for k==1, O(n log n) k>=2.
package main

import (
	"fmt"
	"sort"
)

func smallest(s string, k int) string {
	if k >= 2 {
		b := []byte(s)
		sort.Slice(b, func(i, j int) bool { return b[i] < b[j] })
		return string(b)
	}
	best := s
	n := len(s)
	for i := 1; i < n; i++ {
		rot := s[i:] + s[:i]
		if rot < best {
			best = rot
		}
	}
	return best
}

func main() {
	fmt.Println(smallest("daily", 1))
}
