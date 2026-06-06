// Day 1622: Longest substring with at most k distinct characters.
// Sliding window with a count map. Time O(n), Space O(k).
package main

import "fmt"

func longestKDistinct(s string, k int) string {
	if k <= 0 {
		return ""
	}
	cnt := make(map[byte]int)
	left, bestL, bestLen := 0, 0, 0
	for right := 0; right < len(s); right++ {
		cnt[s[right]]++
		for len(cnt) > k {
			cnt[s[left]]--
			if cnt[s[left]] == 0 {
				delete(cnt, s[left])
			}
			left++
		}
		if right-left+1 > bestLen {
			bestLen = right - left + 1
			bestL = left
		}
	}
	return s[bestL : bestL+bestLen]
}

func main() {
	fmt.Printf("\"%s\"\n", longestKDistinct("abcba", 2))
}
