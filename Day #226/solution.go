// Alien Dictionary: build precedence graph from adjacent words, topological sort (Kahn's BFS).
// Time: O(total characters), Space: O(unique letters + edges).
package main

import (
	"fmt"
	"sort"
)

func alienOrder(words []string) []byte {
	adj := map[byte]map[byte]bool{}
	indeg := map[byte]int{}
	for _, w := range words {
		for i := 0; i < len(w); i++ {
			c := w[i]
			if adj[c] == nil {
				adj[c] = map[byte]bool{}
			}
			if _, ok := indeg[c]; !ok {
				indeg[c] = 0
			}
		}
	}
	for i := 0; i+1 < len(words); i++ {
		a, b := words[i], words[i+1]
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
	var q []byte
	for c, d := range indeg {
		if d == 0 {
			q = append(q, c)
		}
	}
	sort.Slice(q, func(i, j int) bool { return q[i] < q[j] })
	var order []byte
	for len(q) > 0 {
		c := q[0]
		q = q[1:]
		order = append(order, c)
		var nxs []byte
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
	fmt.Print("[")
	for i, c := range o {
		fmt.Printf("'%c'", c)
		if i+1 < len(o) {
			fmt.Print(", ")
		}
	}
	fmt.Println("]")
}
