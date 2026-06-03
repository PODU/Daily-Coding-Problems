// Bijective char mapping s1->s2: lengths equal + consistent forward map + injective (no two s1 chars share an s2 char).
// Time O(n), Space O(1) (fixed 256-size maps).
package main

import "fmt"

func bijectiveMap(s1, s2 string) bool {
	if len(s1) != len(s2) {
		return false
	}
	var fwd, rev [256]int
	for i := range fwd {
		fwd[i], rev[i] = -1, -1
	}
	for i := 0; i < len(s1); i++ {
		a, b := s1[i], s2[i]
		if fwd[a] == -1 && rev[b] == -1 {
			fwd[a], rev[b] = int(b), int(a)
		} else if fwd[a] != int(b) || rev[b] != int(a) {
			return false
		}
	}
	return true
}

func main() {
	fmt.Println(bijectiveMap("abc", "bcd"))
	fmt.Println(bijectiveMap("foo", "bar"))
}
