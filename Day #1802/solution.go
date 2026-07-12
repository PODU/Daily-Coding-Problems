// Palindrome pairs: map word->index, split each word, match palindromic halves.
// Time O(N*L^2), Space O(N*L).
package main

import (
	"fmt"
	"sort"
	"strings"
)

func isPal(s string) bool {
	for l, r := 0, len(s)-1; l < r; l, r = l+1, r-1 {
		if s[l] != s[r] {
			return false
		}
	}
	return true
}

func reverse(s string) string {
	b := []byte(s)
	for l, r := 0, len(b)-1; l < r; l, r = l+1, r-1 {
		b[l], b[r] = b[r], b[l]
	}
	return string(b)
}

func palindromePairs(words []string) [][2]int {
	idx := make(map[string]int)
	for i, w := range words {
		idx[w] = i
	}
	seen := make(map[[2]int]bool)
	var res [][2]int
	for i, w := range words {
		n := len(w)
		for j := 0; j <= n; j++ {
			prefix, suffix := w[:j], w[j:]
			if isPal(prefix) {
				rs := reverse(suffix)
				if k, ok := idx[rs]; ok && k != i {
					p := [2]int{k, i}
					if !seen[p] {
						seen[p] = true
						res = append(res, p)
					}
				}
			}
			if len(suffix) > 0 && isPal(suffix) {
				rp := reverse(prefix)
				if k, ok := idx[rp]; ok && k != i {
					p := [2]int{i, k}
					if !seen[p] {
						seen[p] = true
						res = append(res, p)
					}
				}
			}
		}
	}
	sort.Slice(res, func(a, b int) bool {
		if res[a][0] != res[b][0] {
			return res[a][0] < res[b][0]
		}
		return res[a][1] < res[b][1]
	})
	return res
}

func main() {
	words := []string{"code", "edoc", "da", "d"}
	res := palindromePairs(words)
	parts := make([]string, len(res))
	for i, p := range res {
		parts[i] = fmt.Sprintf("(%d, %d)", p[0], p[1])
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
