// Day 1203: Alien dictionary - derive letter order from sorted words.
// Build precedence graph from adjacent word diffs, Kahn topological sort. Time O(total chars), Space O(1) alphabet.
package main

import (
	"fmt"
	"sort"
	"strings"
)

func alienOrder(words []string) []rune {
	adj := map[rune]map[rune]bool{}
	indeg := map[rune]int{}
	for _, w := range words {
		for _, c := range w {
			if _, ok := adj[c]; !ok {
				adj[c] = map[rune]bool{}
				indeg[c] = 0
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
	var q []rune
	for c, d := range indeg {
		if d == 0 {
			q = append(q, c)
		}
	}
	sort.Slice(q, func(i, j int) bool { return q[i] < q[j] })
	var order []rune
	for len(q) > 0 {
		c := q[0]
		q = q[1:]
		order = append(order, c)
		var nxs []rune
		for nx := range adj[c] {
			nxs = append(nxs, nx)
		}
		sort.Slice(nxs, func(i, j int) bool { return nxs[i] < nxs[j] })
		for _, nx := range nxs {
			indeg[nx]--
			if indeg[nx] == 0 {
				q = append(q, nx)
			}
		}
	}
	return order
}

func main() {
	words := []string{"xww", "wxyz", "wxyw", "ywx", "ywz"}
	o := alienOrder(words)
	parts := make([]string, len(o))
	for i, c := range o {
		parts[i] = "'" + string(c) + "'"
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]") // ['x', 'z', 'w', 'y']
}
