// Word break reconstruction via DP with backpointers. O(n^2) time, O(n) space.
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
	back := make([]int, n+1)
	for i := range back {
		back[i] = -2 // -2 unreachable
	}
	back[0] = -1
	for i := 1; i <= n; i++ {
		for j := 0; j < i; j++ {
			if back[j] != -2 && dict[s[j:i]] {
				back[i] = j
				break
			}
		}
	}
	if back[n] == -2 {
		return nil
	}
	var res []string
	for i := n; i > 0; i = back[i] {
		res = append([]string{s[back[i]:i]}, res...)
	}
	return res
}

func main() {
	words := []string{"quick", "brown", "the", "fox"}
	s := "thequickbrownfox"
	res := wordBreak(s, words)
	parts := make([]string, len(res))
	for i, w := range res {
		parts[i] = "'" + w + "'"
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
