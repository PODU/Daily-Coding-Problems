// Concatenation of all equal-length words: sliding window per offset (0..L-1).
// Time O(|s| * L), Space O(words). Each word used exactly once.
package main

import (
	"fmt"
	"sort"
)

func findSubstring(s string, words []string) []int {
	res := []int{}
	if len(words) == 0 {
		return res
	}
	L := len(words[0])
	k := len(words)
	n := len(s)
	if L*k > n {
		return res
	}
	need := map[string]int{}
	for _, w := range words {
		need[w]++
	}
	for off := 0; off < L; off++ {
		left := off
		count := 0
		win := map[string]int{}
		for j := off; j+L <= n; j += L {
			w := s[j : j+L]
			if _, ok := need[w]; ok {
				win[w]++
				count++
				for win[w] > need[w] {
					lw := s[left : left+L]
					win[lw]--
					left += L
					count--
				}
				if count == k {
					res = append(res, left)
					lw := s[left : left+L]
					win[lw]--
					left += L
					count--
				}
			} else {
				win = map[string]int{}
				count = 0
				left = j + L
			}
		}
	}
	sort.Ints(res)
	return res
}

func main() {
	fmt.Println(findSubstring("dogcatcatcodecatdog", []string{"cat", "dog"}))
	fmt.Println(findSubstring("barfoobazbitbyte", []string{"dog", "cat"}))
}
