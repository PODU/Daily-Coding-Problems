// Day 1854: Shortest substring containing all chars in a set.
// Sliding window with a need-counter; expand right, contract left while valid. O(n) time, O(|set|) space.
package main

import "fmt"

// returns (substring, found)
func shortestSubstring(s string, need map[rune]bool) (string, bool) {
	want := map[byte]int{}
	for c := range need {
		want[byte(c)]++
	}
	required := len(want)
	win := map[byte]int{}
	formed := 0
	bestLen := 1 << 30
	bestL := 0
	l := 0
	for r := 0; r < len(s); r++ {
		c := s[r]
		if _, ok := want[c]; ok {
			win[c]++
			if win[c] == want[c] {
				formed++
			}
		}
		for formed == required {
			if r-l+1 < bestLen {
				bestLen = r - l + 1
				bestL = l
			}
			lc := s[l]
			l++
			if _, ok := want[lc]; ok {
				win[lc]--
				if win[lc] < want[lc] {
					formed--
				}
			}
		}
	}
	if bestLen == 1<<30 {
		return "", false
	}
	return s[bestL : bestL+bestLen], true
}

func main() {
	need := map[rune]bool{'a': true, 'e': true, 'i': true}
	res, ok := shortestSubstring("figehaeci", need)
	if ok {
		fmt.Println(res) // aeci
	} else {
		fmt.Println("null")
	}
}
