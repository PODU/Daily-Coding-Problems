// Word circle = Eulerian circuit in graph where each word is an edge first->last char.
// Check in==out degrees, then Hierholzer to build chain. O(V+E).
package main

import (
	"fmt"
	"strings"
)

type edge struct {
	word string
	dst  byte
}

func solve(words []string) string {
	adj := map[byte][]edge{}
	indeg := map[byte]int{}
	outdeg := map[byte]int{}
	idx := map[byte]int{}
	for _, w := range words {
		a, b := w[0], w[len(w)-1]
		adj[a] = append(adj[a], edge{w, b})
		outdeg[a]++
		indeg[b]++
	}
	nodes := map[byte]bool{}
	for k := range indeg {
		nodes[k] = true
	}
	for k := range outdeg {
		nodes[k] = true
	}
	for c := range nodes {
		if indeg[c] != outdeg[c] {
			return "No circle"
		}
	}

	start := words[0][0]
	st := []byte{start}
	edgeStack := []string{}
	circuit := []string{}
	for len(st) > 0 {
		u := st[len(st)-1]
		if idx[u] < len(adj[u]) {
			e := adj[u][idx[u]]
			idx[u]++
			st = append(st, e.dst)
			edgeStack = append(edgeStack, e.word)
		} else {
			st = st[:len(st)-1]
			if len(edgeStack) > 0 {
				circuit = append(circuit, edgeStack[len(edgeStack)-1])
				edgeStack = edgeStack[:len(edgeStack)-1]
			}
		}
	}
	if len(circuit) != len(words) {
		return "No circle"
	}
	for i, j := 0, len(circuit)-1; i < j; i, j = i+1, j-1 {
		circuit[i], circuit[j] = circuit[j], circuit[i]
	}
	return strings.Join(circuit, " --> ") + " --> " + circuit[0]
}

func main() {
	words := []string{"chair", "height", "racket", "touch", "tunic"}
	fmt.Println(solve(words))
}
