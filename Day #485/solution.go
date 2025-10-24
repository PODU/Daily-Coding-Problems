// Day 485: Longest substring with at most k distinct characters.
// Sliding window + count map; expand right, shrink left when distinct > k. Time O(n), Space O(k).
package main

import "fmt"

func longestKDistinct(s string, k int) int {
	if k <= 0 {
		return 0
	}
	count := make(map[byte]int)
	left, best := 0, 0
	for right := 0; right < len(s); right++ {
		count[s[right]]++
		for len(count) > k {
			lc := s[left]
			count[lc]--
			if count[lc] == 0 {
				delete(count, lc)
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
