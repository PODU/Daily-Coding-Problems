// Longest substring with at most k distinct characters via a sliding window
// with a char-count map. Time O(n), Space O(k).
package main

import "fmt"

func longestKDistinct(s string, k int) string {
	cnt := map[byte]int{}
	left, bestStart, bestLen := 0, 0, 0
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
			bestStart = left
		}
	}
	return s[bestStart : bestStart+bestLen]
}

func main() {
	sub := longestKDistinct("abcba", 2)
	fmt.Printf("The longest substring with k distinct characters is %q.\n", sub)
}
