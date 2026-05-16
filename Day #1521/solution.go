// Find all anagram start indices of W in S.
// Sliding window + 26-letter freq + match counter. Time O(|S|), Space O(1).
package main

import (
	"fmt"
	"strings"
)

func findAnagrams(W, S string) []int {
	n, m := len(S), len(W)
	res := []int{}
	if m == 0 || m > n {
		return res
	}
	var need, win [26]int
	for i := 0; i < m; i++ {
		need[W[i]-'a']++
	}
	matches := 0
	for i := 0; i < 26; i++ {
		if need[i] == 0 {
			matches++
		}
	}
	for i := 0; i < n; i++ {
		add := S[i] - 'a'
		win[add]++
		if win[add] == need[add] {
			matches++
		} else if win[add] == need[add]+1 {
			matches--
		}
		if i >= m {
			rem := S[i-m] - 'a'
			win[rem]--
			if win[rem] == need[rem] {
				matches++
			} else if win[rem] == need[rem]-1 {
				matches--
			}
		}
		if i >= m-1 && matches == 26 {
			res = append(res, i-m+1)
		}
	}
	return res
}

func main() {
	idx := findAnagrams("ab", "abxaba")
	parts := make([]string, len(idx))
	for i, v := range idx {
		parts[i] = fmt.Sprintf("%d", v)
	}
	fmt.Println(strings.Join(parts, ", "))
}
