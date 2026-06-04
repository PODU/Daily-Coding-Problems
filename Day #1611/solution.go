// Orderly Queue: move one of the first k letters to the end repeatedly; find lexicographically smallest.
// k==1 -> smallest rotation; k>=2 -> sorted ascending. Time O(n^2) (k==1) / O(n log n), Space O(n).
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
	// k == 1: smallest rotation
	best := s
	for i := 1; i < len(s); i++ {
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
