// Day 771: One-to-one (bijective) character mapping between s1 and s2.
// Track forward and reverse maps; reject conflicts. O(n) time, O(1) space.
package main

import "fmt"

func isOneToOne(s1, s2 string) bool {
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
	fmt.Println(isOneToOne("abc", "bcd")) // true
	fmt.Println(isOneToOne("foo", "bar")) // false
}
