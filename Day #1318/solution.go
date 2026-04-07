// Move one of first k chars to the end, unlimited times.
// k==1: only rotations reachable -> smallest rotation. k>=2: any order -> sort.
// Time O(n^2) for k==1 (rotation scan), O(n log n) for k>=2.
package main

import (
	"fmt"
	"sort"
)

func smallestString(s string, k int) string {
	if k >= 2 {
		b := []byte(s)
		sort.Slice(b, func(i, j int) bool { return b[i] < b[j] })
		return string(b)
	}
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
	fmt.Println(smallestString("daily", 1)) // ailyd
}
