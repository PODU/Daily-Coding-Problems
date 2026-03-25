// Day 1265: Reconstruct a sentence from a space-free string and a dictionary.
// DP over prefixes storing one valid breakpoint. O(n^2) time (avg), O(n) space.
package main

import "fmt"

func wordBreak(s string, words []string) ([]string, bool) {
	dict := map[string]bool{}
	for _, w := range words {
		dict[w] = true
	}
	n := len(s)
	back := make([]int, n+1)
	for i := range back {
		back[i] = -2
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
		return nil, false
	}
	var res []string
	for i := n; i > 0; i = back[i] {
		res = append([]string{s[back[i]:i]}, res...)
	}
	return res, true
}

func main() {
	res, ok := wordBreak("thequickbrownfox", []string{"quick", "brown", "the", "fox"})
	if !ok {
		fmt.Println("null")
	} else {
		fmt.Println(res)
	}
}
