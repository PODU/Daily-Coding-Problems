// Day 1414: does a one-to-one (bijective) char mapping s1 -> s2 exist?
// Approach: enforce a consistent forward map AND injective (no two srcs share a target). O(n).
package main

import "fmt"

func canMap(s1, s2 string) bool {
	if len(s1) != len(s2) {
		return false
	}
	fwd := map[byte]byte{}
	rev := map[byte]byte{}
	for i := 0; i < len(s1); i++ {
		a, b := s1[i], s2[i]
		if v, ok := fwd[a]; ok && v != b {
			return false
		}
		if v, ok := rev[b]; ok && v != a {
			return false
		}
		fwd[a] = b
		rev[b] = a
	}
	return true
}

func main() {
	fmt.Println(canMap("abc", "bcd")) // true
	fmt.Println(canMap("foo", "bar")) // false
}
