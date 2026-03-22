// Concatenation substring indices via sliding window over wordLen offsets with hashmap counts. O(n*wordLen) time, O(m) space.
package main

import (
	"fmt"
	"sort"
	"strconv"
	"strings"
)

func findSubstring(s string, words []string) []int {
	res := []int{}
	if len(words) == 0 || len(s) == 0 {
		return res
	}
	wordLen := len(words[0])
	numWords := len(words)
	windowLen := wordLen * numWords
	if len(s) < windowLen {
		return res
	}

	need := map[string]int{}
	for _, w := range words {
		need[w]++
	}

	for offset := 0; offset < wordLen; offset++ {
		window := map[string]int{}
		count := 0
		left := offset
		for right := offset; right+wordLen <= len(s); right += wordLen {
			word := s[right : right+wordLen]
			if _, ok := need[word]; ok {
				window[word]++
				count++
				for window[word] > need[word] {
					lw := s[left : left+wordLen]
					window[lw]--
					count--
					left += wordLen
				}
				if count == numWords {
					res = append(res, left)
					lw := s[left : left+wordLen]
					window[lw]--
					count--
					left += wordLen
				}
			} else {
				window = map[string]int{}
				count = 0
				left = right + wordLen
			}
		}
	}
	sort.Ints(res)
	return res
}

func main() {
	s := "dogcatcatcodecatdog"
	words := []string{"cat", "dog"}
	res := findSubstring(s, words)
	parts := make([]string, len(res))
	for i, v := range res {
		parts[i] = strconv.Itoa(v)
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
