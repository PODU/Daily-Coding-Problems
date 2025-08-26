// Substring concatenation of all words: sliding window over wordLen offsets with hash-map counts.
// Time O(n * wordLen), Space O(words * wordLen).
package main

import (
	"fmt"
	"sort"
	"strings"
)

func findSubstring(s string, words []string) []int {
	res := []int{}
	if len(words) == 0 || len(s) == 0 {
		return res
	}
	wl, nw := len(words[0]), len(words)
	total, n := wl*nw, len(s)
	if total > n {
		return res
	}
	need := map[string]int{}
	for _, w := range words {
		need[w]++
	}
	for i := 0; i < wl; i++ {
		left, count := i, 0
		window := map[string]int{}
		for j := i; j+wl <= n; j += wl {
			w := s[j : j+wl]
			if _, ok := need[w]; ok {
				window[w]++
				count++
				for window[w] > need[w] {
					lw := s[left : left+wl]
					window[lw]--
					count--
					left += wl
				}
				if count == nw {
					res = append(res, left)
					lw := s[left : left+wl]
					window[lw]--
					count--
					left += wl
				}
			} else {
				window = map[string]int{}
				count = 0
				left = j + wl
			}
		}
	}
	sort.Ints(res)
	return res
}

func toStr(v []int) string {
	parts := make([]string, len(v))
	for i, x := range v {
		parts[i] = fmt.Sprintf("%d", x)
	}
	return "[" + strings.Join(parts, ", ") + "]"
}

func main() {
	fmt.Println(toStr(findSubstring("dogcatcatcodecatdog", []string{"cat", "dog"})))
	fmt.Println(toStr(findSubstring("barfoobazbitbyte", []string{"dog", "cat"})))
}
