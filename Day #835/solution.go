// Day 835: Shortest substring containing all chars in a set.
// Sliding-window min-window: expand right, contract left while valid. O(N) time, O(K) space.
package main

import "fmt"

func shortestSubstring(s string, charset map[rune]bool) (string, bool) {
	need := map[byte]int{}
	for c := range charset {
		need[byte(c)] = 1
	}
	have := map[byte]int{}
	required := len(need)
	formed, left, bestL, bestLen := 0, 0, 0, 1<<30
	found := false
	for right := 0; right < len(s); right++ {
		ch := s[right]
		if _, ok := need[ch]; ok {
			have[ch]++
			if have[ch] == need[ch] {
				formed++
			}
		}
		for formed == required {
			if right-left+1 < bestLen {
				bestLen = right - left + 1
				bestL = left
				found = true
			}
			lc := s[left]
			if _, ok := need[lc]; ok {
				have[lc]--
				if have[lc] < need[lc] {
					formed--
				}
			}
			left++
		}
	}
	if found {
		return s[bestL : bestL+bestLen], true
	}
	return "", false
}

func main() {
	charset := map[rune]bool{'a': true, 'e': true, 'i': true}
	res, ok := shortestSubstring("figehaeci", charset)
	if ok {
		fmt.Println(res)
	} else {
		fmt.Println("null")
	}
}
