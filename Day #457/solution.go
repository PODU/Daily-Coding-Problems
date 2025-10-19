// Day 457: All start indices in S that are anagrams of W.
// Fixed-size sliding window of char counts. Time O(|S|), Space O(1).
package main

import (
	"fmt"
	"strconv"
	"strings"
)

func anagramIndices(w, s string) []int {
	res := []int{}
	m, n := len(w), len(s)
	if m == 0 || m > n {
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
		if i >= m-1 && need == win {
			res = append(res, i-m+1)
		}
	}
	return res
}

func main() {
	r := anagramIndices("ab", "abxaba")
	parts := make([]string, len(r))
	for i, v := range r {
		parts[i] = strconv.Itoa(v)
	}
	fmt.Println(strings.Join(parts, ", ")) // 0, 3, 4
}
