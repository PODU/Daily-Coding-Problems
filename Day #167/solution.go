// Palindrome pairs: hash map of reversed words; for each word check prefix/suffix palindrome splits. O(n*k^2) time, O(n*k) space.
package main

import (
	"fmt"
	"sort"
	"strings"
)

func isPalin(s string, i, j int) bool {
	for i < j {
		if s[i] != s[j] {
			return false
		}
		i++
		j--
	}
	return true
}

func reverse(s string) string {
	b := []byte(s)
	for i, j := 0, len(b)-1; i < j; i, j = i+1, j-1 {
		b[i], b[j] = b[j], b[i]
	}
	return string(b)
}

func main() {
	words := []string{"code", "edoc", "da", "d"}
	rev := make(map[string]int)
	for i, w := range words {
		rev[reverse(w)] = i
	}

	seen := make(map[[2]int]bool)
	var res [][2]int
	add := func(a, b int) {
		p := [2]int{a, b}
		if !seen[p] {
			seen[p] = true
			res = append(res, p)
		}
	}

	for i, w := range words {
		n := len(w)
		for cut := 0; cut <= n; cut++ {
			if isPalin(w, 0, cut-1) {
				suf := w[cut:]
				if j, ok := rev[suf]; ok && j != i {
					add(j, i)
				}
			}
			if cut < n && isPalin(w, cut, n-1) {
				pre := w[:cut]
				if j, ok := rev[pre]; ok && j != i {
					add(i, j)
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

	parts := make([]string, len(res))
	for i, p := range res {
		parts[i] = fmt.Sprintf("(%d, %d)", p[0], p[1])
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
