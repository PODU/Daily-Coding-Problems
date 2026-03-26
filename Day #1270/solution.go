// Day 1270: Find all start indices in S that are anagrams of W.
// Fixed-size sliding window with a 26-length count. O(|S|) time.
package main

import (
	"fmt"
	"strconv"
	"strings"
)

func findAnagrams(w, s string) []int {
	var res []int
	m, n := len(w), len(s)
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
	res := findAnagrams("ab", "abxaba")
	strs := make([]string, len(res))
	for i, v := range res {
		strs[i] = strconv.Itoa(v)
	}
	fmt.Println(strings.Join(strs, ", "))
}
