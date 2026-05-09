// Day 1497: Step words. Dict word w is a step word of s if len(w)==len(s)+1
// and multiset(s) subset of multiset(w). Char-count comparison.
// Time O(D*L), Space O(1) (26-letter alphabet).
package main

import "fmt"

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
			diff := cnt[i] - base[i]
			if diff < 0 {
				ok = false
				break
			}
			extra += diff
		}
		if ok && extra == 1 {
			res = append(res, w)
		}
	}
	return res
}

func main() {
	input := "APPLE"
	dict := []string{"APPEAL", "APPLE", "BANANA", "PLEASE", "APPEALS"}
	res := stepWords(input, dict)
	fmt.Print("[")
	for i, w := range res {
		fmt.Printf("\"%s\"", w)
		if i+1 < len(res) {
			fmt.Print(", ")
		}
	}
	fmt.Println("]")
}
