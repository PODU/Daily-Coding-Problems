// Day 111: Anagram substrings via fixed sliding window of char counts. O(|S|).
package main

import (
	"fmt"
	"strconv"
	"strings"
)

func findAnagrams(s, w string) []int {
	res := []int{}
	n, m := len(s), len(w)
	if m > n {
		return res
	}
	var need, win [256]int
	for i := 0; i < m; i++ {
		need[w[i]]++
	}
	for i := 0; i < n; i++ {
		win[s[i]]++
		if i >= m {
			win[s[i-m]]--
		}
		if i >= m-1 && win == need {
			res = append(res, i-m+1)
		}
	}
	return res
}

func main() {
	parts := []string{}
	for _, v := range findAnagrams("abxaba", "ab") {
		parts = append(parts, strconv.Itoa(v))
	}
	fmt.Println(strings.Join(parts, ", "))
}
