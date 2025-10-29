// Day 510: All index pairs (i,j) where words[i]+words[j] is a palindrome.
// Hash map of reversed words + prefix/suffix palindrome checks. Time O(N*L^2).
package main

import (
	"fmt"
	"sort"
	"strings"
)

func reverse(s string) string {
	r := []byte(s)
	for i, j := 0, len(r)-1; i < j; i, j = i+1, j-1 {
		r[i], r[j] = r[j], r[i]
	}
	return string(r)
}

func isPal(s string) bool {
	for l, r := 0, len(s)-1; l < r; l, r = l+1, r-1 {
		if s[l] != s[r] {
			return false
		}
	}
	return true
}

func palindromePairs(words []string) [][2]int {
	rev := make(map[string]int)
	for i, w := range words {
		rev[reverse(w)] = i
	}
	seen := make(map[[2]int]bool)
	var result [][2]int
	add := func(p [2]int) {
		if !seen[p] {
			seen[p] = true
			result = append(result, p)
		}
	}
	for i, w := range words {
		n := len(w)
		for cut := 0; cut <= n; cut++ {
			if isPal(w[:cut]) {
				if j, ok := rev[w[cut:]]; ok && j != i {
					add([2]int{j, i})
				}
			}
			if cut != n && isPal(w[cut:]) {
				if j, ok := rev[w[:cut]]; ok && j != i {
					add([2]int{i, j})
				}
			}
		}
	}
	sort.Slice(result, func(a, b int) bool {
		if result[a][0] != result[b][0] {
			return result[a][0] < result[b][0]
		}
		return result[a][1] < result[b][1]
	})
	return result
}

func main() {
	words := []string{"code", "edoc", "da", "d"}
	pairs := palindromePairs(words)
	parts := make([]string, len(pairs))
	for k, p := range pairs {
		parts[k] = fmt.Sprintf("(%d, %d)", p[0], p[1])
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
