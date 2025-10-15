// Day 437: Shortest substring containing all chars of a set via sliding window.
// O(N) time, O(set) space.
package main

import "fmt"

func shortestSubstring(s string, chars map[rune]bool) (string, bool) {
	if len(chars) == 0 {
		return "", true
	}
	need := make(map[rune]int)
	for c := range chars {
		need[c] = 0
	}
	required := len(chars)
	formed := 0
	bestLen := -1
	bestL := 0
	l := 0
	rs := []rune(s)
	for r := 0; r < len(rs); r++ {
		c := rs[r]
		if _, ok := need[c]; ok {
			if need[c] == 0 {
				formed++
			}
			need[c]++
		}
		for formed == required {
			if bestLen == -1 || r-l+1 < bestLen {
				bestLen = r - l + 1
				bestL = l
			}
			lc := rs[l]
			if _, ok := need[lc]; ok {
				need[lc]--
				if need[lc] == 0 {
					formed--
				}
			}
			l++
		}
	}
	if bestLen == -1 {
		return "", false
	}
	return string(rs[bestL : bestL+bestLen]), true
}

func main() {
	chars := map[rune]bool{'a': true, 'e': true, 'i': true}
	res, ok := shortestSubstring("figehaeci", chars)
	if !ok {
		fmt.Println("null")
	} else {
		fmt.Printf("\"%s\"\n", res) // "aeci"
	}
}
