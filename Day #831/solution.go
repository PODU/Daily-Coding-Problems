// Day 831: All start indices of substrings that are a concatenation of every word once.
// Sliding window per offset 0..L-1 with hashmap word counts. O(n * L) ~ O(n) total.
package main

import (
	"fmt"
	"sort"
	"strconv"
	"strings"
)

func format(v []int) string {
	parts := make([]string, len(v))
	for i, x := range v {
		parts[i] = strconv.Itoa(x)
	}
	return "[" + strings.Join(parts, ", ") + "]"
}

func findSubstring(s string, words []string) []int {
	res := []int{}
	if len(words) == 0 || len(s) == 0 {
		return res
	}
	L := len(words[0])
	m := len(words)
	n := len(s)
	if L*m > n {
		return res
	}
	need := make(map[string]int)
	for _, w := range words {
		need[w]++
	}

	for offset := 0; offset < L; offset++ {
		left := offset
		count := 0
		have := make(map[string]int)
		for right := offset; right+L <= n; right += L {
			w := s[right : right+L]
			if _, ok := need[w]; ok {
				have[w]++
				count++
				for have[w] > need[w] {
					lw := s[left : left+L]
					have[lw]--
					left += L
					count--
				}
				if count == m {
					res = append(res, left)
					lw := s[left : left+L]
					have[lw]--
					left += L
					count--
				}
			} else {
				have = make(map[string]int)
				count = 0
				left = right + L
			}
		}
	}
	sort.Ints(res)
	return res
}

func main() {
	fmt.Println(format(findSubstring("dogcatcatcodecatdog", []string{"cat", "dog"}))) // [0, 13]
	fmt.Println(format(findSubstring("barfoobazbitbyte", []string{"dog", "cat"})))    // []
}
