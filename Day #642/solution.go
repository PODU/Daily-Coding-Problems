// Day 642: Step words (add one letter + anagram).
// Approach: a dict word qualifies if len == len(word)+1 and its letter counts
// minus the input's are all >= 0 with exactly one extra letter.
// Time: O(D * L), Space: O(1) (26-letter counts).
package main

import "fmt"

func isStep(word, cand string) bool {
	if len(cand) != len(word)+1 {
		return false
	}
	var cnt [26]int
	for _, c := range cand {
		cnt[c-'A']++
	}
	for _, c := range word {
		cnt[c-'A']--
		if cnt[c-'A'] < 0 {
			return false
		}
	}
	extra := 0
	for _, v := range cnt {
		extra += v
	}
	return extra == 1
}

func stepWords(word string, dict []string) []string {
	res := []string{}
	for _, w := range dict {
		if isStep(word, w) {
			res = append(res, w)
		}
	}
	return res
}

func main() {
	word := "APPLE"
	dict := []string{"APPEAL", "APPEAR", "PEAR", "APPLES", "PALE"}
	fmt.Println(stepWords(word, dict)) // [APPEAL APPLES]
}
