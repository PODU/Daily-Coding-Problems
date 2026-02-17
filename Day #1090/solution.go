// Palindrome pairs: hash words; for each split test palindromic remainder + reversed counterpart.
// Time O(n*k^2), Space O(n*k).
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
	r := []byte(s)
	for i, j := 0, len(r)-1; i < j; i, j = i+1, j-1 {
		r[i], r[j] = r[j], r[i]
	}
	return string(r)
}

func main() {
	words := []string{"code", "edoc", "da", "d"}
	d := map[string]int{}
	for i, w := range words {
		d[w] = i
	}
	type P struct{ a, b int }
	seen := map[P]bool{}
	var res []P
	add := func(p P) {
		if !seen[p] {
			seen[p] = true
			res = append(res, p)
		}
	}
	for i, w := range words {
		L := len(w)
		for j := 0; j <= L; j++ {
			if isPal(w, 0, j-1) {
				if k, ok := d[reverse(w[j:])]; ok && k != i {
					add(P{k, i})
				}
			}
			if j != L && isPal(w, j, L-1) {
				if k, ok := d[reverse(w[:j])]; ok && k != i {
					add(P{i, k})
				}
			}
		}
	}
	sort.Slice(res, func(i, j int) bool {
		if res[i].a != res[j].a {
			return res[i].a < res[j].a
		}
		return res[i].b < res[j].b
	})
	parts := make([]string, len(res))
	for i, p := range res {
		parts[i] = fmt.Sprintf("(%d, %d)", p.a, p.b)
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
