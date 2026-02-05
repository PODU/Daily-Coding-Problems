// Day 1023: Alien dictionary - order of letters from sorted words.
// Approach: build precedence graph from adjacent words, Kahn's topological sort.
// Time O(total chars + V + E), Space O(V + E).
package main

import (
	"fmt"
	"sort"
	"strings"
)

func alienOrder(words []string) []rune {
	appear := []rune{}
	adj := map[rune]map[rune]bool{}
	indeg := map[rune]int{}
	for _, w := range words {
		for _, c := range w {
			if _, ok := indeg[c]; !ok {
				indeg[c] = 0
				appear = append(appear, c)
				adj[c] = map[rune]bool{}
			}
		}
	}

	for i := 0; i+1 < len(words); i++ {
		a, b := []rune(words[i]), []rune(words[i+1])
		n := len(a)
		if len(b) < n {
			n = len(b)
		}
		for j := 0; j < n; j++ {
			if a[j] != b[j] {
				if !adj[a[j]][b[j]] {
					adj[a[j]][b[j]] = true
					indeg[b[j]]++
				}
				break
			}
		}
	}

	q := []rune{}
	for _, c := range appear {
		if indeg[c] == 0 {
			q = append(q, c)
		}
	}
	res := []rune{}
	for len(q) > 0 {
		c := q[0]
		q = q[1:]
		res = append(res, c)
		nbs := []rune{}
		for nb := range adj[c] {
			nbs = append(nbs, nb)
		}
		sort.Slice(nbs, func(i, j int) bool { return nbs[i] < nbs[j] })
		for _, nb := range nbs {
			indeg[nb]--
			if indeg[nb] == 0 {
				q = append(q, nb)
			}
		}
	}
	return res
}

func main() {
	words := []string{"xww", "wxyz", "wxyw", "ywx", "ywz"}
	parts := []string{}
	for _, c := range alienOrder(words) {
		parts = append(parts, "'"+string(c)+"'")
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
