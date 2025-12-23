// Step words: a dict word qualifies if len==word.len+1 and word's letter
// multiset is a subset leaving exactly one extra letter. O(dict*wordLen) time, O(1) space.
package main

import (
	"fmt"
	"strings"
)

func stepWords(word string, dict []string) []string {
	var base [26]int
	for _, c := range word {
		base[c-'A']++
	}
	res := []string{}
	for _, w := range dict {
		if len(w) != len(word)+1 {
			continue
		}
		var cnt [26]int
		for _, c := range w {
			cnt[c-'A']++
		}
		extra, ok := 0, true
		for i := 0; i < 26; i++ {
			d := cnt[i] - base[i]
			if d < 0 {
				ok = false
				break
			}
			extra += d
		}
		if ok && extra == 1 {
			res = append(res, w)
		}
	}
	return res
}

func main() {
	word := "APPLE"
	dict := []string{"APPEAL", "BANANA", "ORANGE", "GRAPE"}
	fmt.Println(strings.Join(stepWords(word, dict), " "))
}
