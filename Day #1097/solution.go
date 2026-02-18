// Day 1097: Smallest string by moving one of first k letters to the end.
// k==1 -> min rotation; k>=2 -> any permutation reachable -> sorted string.
// Time: O(N^2) for k==1, O(N log N) for k>=2. Space: O(N).
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
	cur := s
	for i := 0; i < len(s); i++ {
		cur = cur[1:] + cur[0:1]
		if cur < best {
			best = cur
		}
	}
	return best
}

func main() {
	fmt.Println(smallest("daily", 1)) // ailyd
}
