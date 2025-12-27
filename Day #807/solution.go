// Day 807: Longest substring with at most k distinct characters.
// Sliding window + char count map; shrink left when distinct > k. Time O(N), Space O(k).
package main

import "fmt"

func longestKDistinct(s string, k int) int {
	if k == 0 {
		return 0
	}
	cnt := map[byte]int{}
	left, best := 0, 0
	for right := 0; right < len(s); right++ {
		cnt[s[right]]++
		for len(cnt) > k {
			cnt[s[left]]--
			if cnt[s[left]] == 0 {
				delete(cnt, s[left])
			}
			left++
		}
		if right-left+1 > best {
			best = right - left + 1
		}
	}
	return best
}

func main() {
	fmt.Println(longestKDistinct("abcba", 2)) // 3
}
