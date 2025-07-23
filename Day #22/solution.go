// Word Break reconstruction: DP over positions with memoization using a word set.
// Time: O(n^2) substring checks (n = string length), Space: O(n) for memo + recursion.
package main

import (
	"fmt"
	"strings"
)

func wordBreak(s string, words []string) []string {
	dict := make(map[string]bool)
	for _, w := range words {
		dict[w] = true
	}
	n := len(s)
	memo := make([]int, n+1)
	for i := range memo {
		memo[i] = -2
	}

	var solve func(i int) bool
	solve = func(i int) bool {
		if i == n {
			return true
		}
		if memo[i] != -2 {
			return memo[i] != -1
		}
		for j := i + 1; j <= n; j++ {
			if dict[s[i:j]] && solve(j) {
				memo[i] = j - i
				return true
			}
		}
		memo[i] = -1
		return false
	}

	if !solve(0) {
		return nil
	}
	var res []string
	for i := 0; i < n; {
		res = append(res, s[i:i+memo[i]])
		i += memo[i]
	}
	return res
}

func main() {
	words := []string{"quick", "brown", "the", "fox"}
	s := "thequickbrownfox"
	res := wordBreak(s, words)
	if res == nil {
		fmt.Println("None")
		return
	}
	parts := make([]string, len(res))
	for i, w := range res {
		parts[i] = "'" + w + "'"
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
