// Day 103: Shortest window containing all chars of a set via sliding window with
// a required-count and a "have all" tracker. O(n) time, O(set) space.
package main

import "fmt"

func minWindow(s string, chars []byte) (string, bool) {
	need := map[byte]bool{}
	for _, c := range chars {
		need[c] = true
	}
	if len(need) == 0 {
		return "", true
	}
	count := map[byte]int{}
	have, left, bestLen, bestStart := 0, 0, len(s)+1, 0
	for right := 0; right < len(s); right++ {
		ch := s[right]
		if need[ch] {
			count[ch]++
			if count[ch] == 1 {
				have++
			}
		}
		for have == len(need) {
			if right-left+1 < bestLen {
				bestLen = right - left + 1
				bestStart = left
			}
			lc := s[left]
			if need[lc] {
				count[lc]--
				if count[lc] == 0 {
					have--
				}
			}
			left++
		}
	}
	if bestLen == len(s)+1 {
		return "", false
	}
	return s[bestStart : bestStart+bestLen], true
}

func main() {
	w, _ := minWindow("figehaeci", []byte{'a', 'e', 'i'})
	fmt.Println(w) // aeci
}
