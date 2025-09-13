// Day 266: Step words. A dict word is a step word of `word` if its length is
// one greater and its letter multiset is a superset of `word`'s (diff = 1).
// Time O(D * L) over dictionary; space O(alphabet).
package main

import (
	"fmt"
	"strings"
)

func isStepWord(word, cand string) bool {
	if len(cand) != len(word)+1 {
		return false
	}
	var cnt [26]int
	for _, c := range strings.ToLower(cand) {
		cnt[c-'a']++
	}
	for _, c := range strings.ToLower(word) {
		cnt[c-'a']--
		if cnt[c-'a'] < 0 {
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
	var res []string
	for _, w := range dict {
		if isStepWord(word, w) {
			res = append(res, w)
		}
	}
	return res
}

func main() {
	word := "APPLE"
	dict := []string{"APPEAL", "APPLES", "KELP", "PALE", "APPLE"}
	fmt.Println("[" + strings.Join(stepWords(word, dict), ", ") + "]")
}
