// Bijective char mapping: maintain s1->s2 (consistent) and s2->s1 (injective) maps. Time O(n), Space O(n).
package main

import "fmt"

func bijective(s1, s2 string) bool {
	if len(s1) != len(s2) {
		return false
	}
	fwd := map[byte]byte{}
	bwd := map[byte]byte{}
	for i := 0; i < len(s1); i++ {
		a, b := s1[i], s2[i]
		if v, ok := fwd[a]; ok && v != b {
			return false
		}
		if v, ok := bwd[b]; ok && v != a {
			return false
		}
		fwd[a] = b
		bwd[b] = a
	}
	return true
}

func main() {
	if bijective("abc", "bcd") {
		fmt.Println("true (map a to b, b to c, and c to d)")
	}
	if !bijective("foo", "bar") {
		fmt.Println("false (the o cannot map to two characters)")
	}
}
