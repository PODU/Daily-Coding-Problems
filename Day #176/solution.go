// Bijective char mapping check: track s1->s2 map and set of used s2 chars; reject conflicts.
// Time O(n), Space O(unique chars).
package main

import "fmt"

func isBijective(s1, s2 string) bool {
	if len(s1) != len(s2) {
		return false
	}
	mapping := map[byte]byte{}
	used := map[byte]bool{}
	for i := 0; i < len(s1); i++ {
		a, b := s1[i], s2[i]
		if v, ok := mapping[a]; ok {
			if v != b {
				return false
			}
		} else {
			if used[b] {
				return false
			}
			mapping[a] = b
			used[b] = true
		}
	}
	return true
}

func main() {
	fmt.Println(isBijective("abc", "bcd"))
	fmt.Println(isBijective("foo", "bar"))
}
