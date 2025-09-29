// Day 347: Lexicographically smallest string by moving one of first k letters to the end.
// k==1 -> best rotation; k>=2 -> any permutation reachable, so sorted. Time O(N^2)/O(N log N).
package main

import (
	"fmt"
	"sort"
)

func smallest(s string, k int) string {
	if k == 1 {
		best := s
		for i := 1; i < len(s); i++ {
			rot := s[i:] + s[:i]
			if rot < best {
				best = rot
			}
		}
		return best
	}
	b := []byte(s)
	sort.Slice(b, func(i, j int) bool { return b[i] < b[j] })
	return string(b)
}

func main() {
	fmt.Println(smallest("daily", 1))
}
