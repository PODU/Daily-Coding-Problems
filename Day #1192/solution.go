// Step word: dict word of length len(input)+1 that contains all input letters plus exactly one extra,
// AND is a genuine anagram (rearrangement), not the input with a letter merely appended (no input prefix).
// Compare 26-letter frequency counts: every count >= input's and total diff == 1. O(D * (L + 26)).
package main

import (
	"fmt"
	"strings"
)

func counts(s string) [26]int {
	var c [26]int
	for _, ch := range s {
		c[ch-'A']++
	}
	return c
}

func stepWords(word string, dict []string) []string {
	base := counts(word)
	var res []string
	for _, w := range dict {
		if len(w) != len(word)+1 {
			continue
		}
		cnt := counts(w)
		ok := true
		diff := 0
		for i := 0; i < 26; i++ {
			if cnt[i] < base[i] {
				ok = false
				break
			}
			diff += cnt[i] - base[i]
		}
		if ok && diff == 1 && !strings.HasPrefix(w, word) {
			res = append(res, w)
		}
	}
	return res
}

func main() {
	word := "APPLE"
	dict := []string{"APPEAL", "APPLE", "PEAR", "PALE", "APPEALS", "PAPER", "APPLES", "LAPEL"}
	for _, w := range stepWords(word, dict) {
		fmt.Println(w)
	}
}
