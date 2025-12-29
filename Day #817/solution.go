// Word break via DP with backpointers: dp[i] reachable, prev[i] start of last word. O(n^2) time, O(n) space.
package main

import (
	"fmt"
	"strings"
)

func wordBreak(s string, dict map[string]bool) []string {
	n := len(s)
	prev := make([]int, n+1)
	for i := range prev {
		prev[i] = -2 // -2 = unreachable
	}
	prev[0] = -1
	for i := 1; i <= n; i++ {
		for j := i - 1; j >= 0; j-- { // prefer shortest last word
			if prev[j] != -2 && dict[s[j:i]] {
				prev[i] = j
				break
			}
		}
	}
	if prev[n] == -2 {
		return nil
	}
	var res []string
	for i := n; i > 0; i = prev[i] {
		res = append([]string{s[prev[i]:i]}, res...)
	}
	return res
}

func format(r []string) string {
	if r == nil {
		return "null"
	}
	return "[" + strings.Join(r, ", ") + "]"
}

func main() {
	fmt.Println(format(wordBreak("thequickbrownfox",
		map[string]bool{"quick": true, "brown": true, "the": true, "fox": true})))
	fmt.Println(format(wordBreak("bedbathandbeyond",
		map[string]bool{"bed": true, "bath": true, "bedbath": true, "and": true, "beyond": true})))
}
