// Day 710: Find start indices where s contains a concatenation of all equal-length
// words exactly once. Sliding window over wordLen offsets. Time O(n*wordLen).
package main

import (
	"fmt"
	"sort"
	"strings"
)

func findSubstring(s string, words []string) []int {
	res := []int{}
	if len(words) == 0 {
		return res
	}
	wl, k, n := len(words[0]), len(words), len(s)
	if wl*k > n {
		return res
	}
	need := map[string]int{}
	for _, w := range words {
		need[w]++
	}
	for off := 0; off < wl; off++ {
		left, count := off, 0
		window := map[string]int{}
		for j := off; j+wl <= n; j += wl {
			w := s[j : j+wl]
			if _, ok := need[w]; ok {
				window[w]++
				count++
				for window[w] > need[w] {
					lw := s[left : left+wl]
					window[lw]--
					left += wl
					count--
				}
				if count == k {
					res = append(res, left)
					lw := s[left : left+wl]
					window[lw]--
					left += wl
					count--
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

func fmtIdx(a []int) string {
	parts := make([]string, len(a))
	for i, v := range a {
		parts[i] = fmt.Sprintf("%d", v)
	}
	return "[" + strings.Join(parts, ", ") + "]"
}

func main() {
	fmt.Println(fmtIdx(findSubstring("dogcatcatcodecatdog", []string{"cat", "dog"})))
	fmt.Println(fmtIdx(findSubstring("barfoobazbitbyte", []string{"dog", "cat"})))
}
