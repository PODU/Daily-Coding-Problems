// Day 767: Find all start indices in S that are anagrams of W.
// Sliding window of size |W| with a 26-count array. O(|S|) time, O(1) space.
package main

import (
	"fmt"
	"strings"
)

func findAnagrams(s, w string) []int {
	res := []int{}
	n, m := len(s), len(w)
	if m > n {
		return res
	}
	var need, win [26]int
	for i := 0; i < m; i++ {
		need[w[i]-'a']++
	}
	for i := 0; i < n; i++ {
		win[s[i]-'a']++
		if i >= m {
			win[s[i-m]-'a']--
		}
		if i >= m-1 && win == need {
			res = append(res, i-m+1)
		}
	}
	return res
}

func main() {
	r := findAnagrams("abxaba", "ab")
	parts := make([]string, len(r))
	for i, v := range r {
		parts[i] = fmt.Sprint(v)
	}
	fmt.Println(strings.Join(parts, ", ")) // 0, 3, 4
}
