// Day 1623: Sentence equivalence via synonym set.
// Build symmetric synonym set; compare word-by-word. Time O(W), Space O(S).
package main

import (
	"fmt"
	"strings"
)

func equivalent(a, b string, syns [][2]string) bool {
	pairs := make(map[string]bool)
	for _, p := range syns {
		pairs[p[0]+"\x00"+p[1]] = true
		pairs[p[1]+"\x00"+p[0]] = true
	}
	wa, wb := strings.Fields(a), strings.Fields(b)
	if len(wa) != len(wb) {
		return false
	}
	for i := range wa {
		if wa[i] != wb[i] && !pairs[wa[i]+"\x00"+wb[i]] {
			return false
		}
	}
	return true
}

func main() {
	syns := [][2]string{{"big", "large"}, {"eat", "consume"}}
	eq := equivalent("He wants to eat food.", "He wants to consume food.", syns)
	if eq {
		fmt.Println("The two sentences are equivalent.")
	} else {
		fmt.Println("The two sentences are not equivalent.")
	}
}
