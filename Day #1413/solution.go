// Day 1413: shortest substring of s containing all characters in a set.
// Approach: sliding window, expand right then shrink left while valid. O(n) time, O(k) space.
package main

import "fmt"

func shortestSubstring(s string, want []byte) string {
	need := map[byte]int{}
	for _, c := range want {
		need[c]++
	}
	required := len(need)
	win := map[byte]int{}
	formed := 0
	bestLen := 1 << 30
	bestL := 0
	l := 0
	for r := 0; r < len(s); r++ {
		c := s[r]
		if _, ok := need[c]; ok {
			win[c]++
			if win[c] == need[c] {
				formed++
			}
		}
		for formed == required {
			if r-l+1 < bestLen {
				bestLen = r - l + 1
				bestL = l
			}
			lc := s[l]
			if _, ok := need[lc]; ok {
				win[lc]--
				if win[lc] < need[lc] {
					formed--
				}
			}
			l++
		}
	}
	if bestLen == 1<<30 {
		return "null"
	}
	return s[bestL : bestL+bestLen]
}

func main() {
	fmt.Println(shortestSubstring("figehaeci", []byte{'a', 'e', 'i'})) // aeci
}
