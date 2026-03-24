// Day 1261: Palindrome pairs.
// Hashmap of words + prefix/suffix palindrome splits. O(n*k^2) time, O(n*k) space.
package main

import (
	"fmt"
	"sort"
	"strings"
)

func isPal(s string, i, j int) bool {
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
	r := []rune(s)
	for i, j := 0, len(r)-1; i < j; i, j = i+1, j-1 {
		r[i], r[j] = r[j], r[i]
	}
	return string(r)
}

func palindromePairs(words []string) [][2]int {
	idx := map[string]int{}
	for i, w := range words {
		idx[w] = i
	}
	seen := map[[2]int]bool{}
	var res [][2]int
	for i, w := range words {
		n := len(w)
		for j := 0; j <= n; j++ {
			if isPal(w, 0, j-1) {
				back := reverse(w[j:])
				if k, ok := idx[back]; ok && k != i {
					p := [2]int{k, i}
					if !seen[p] {
						seen[p] = true
						res = append(res, p)
					}
				}
			}
			if j != n && isPal(w, j, n-1) {
				back := reverse(w[:j])
				if k, ok := idx[back]; ok && k != i {
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
	res := palindromePairs([]string{"code", "edoc", "da", "d"})
	parts := make([]string, len(res))
	for i, p := range res {
		parts[i] = fmt.Sprintf("(%d, %d)", p[0], p[1])
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
