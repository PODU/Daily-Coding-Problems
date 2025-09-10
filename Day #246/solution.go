// Words form a circle: model each word as a directed edge first->last char; an Eulerian circuit
// exists iff in-degree==out-degree for every node and edges form one connected component.
// Find the circuit with Hierholzer's algorithm. O(V + E) time and space.
package main

import (
	"fmt"
	"strings"
)

type edge struct {
	to   byte
	word string
}

func circleOrder(words []string) []string {
	adj := map[byte][]edge{}
	indeg := map[byte]int{}
	outdeg := map[byte]int{}
	nodes := map[byte]bool{}
	for _, w := range words {
		a, b := w[0], w[len(w)-1]
		adj[a] = append(adj[a], edge{b, w})
		outdeg[a]++
		indeg[b]++
		nodes[a] = true
		nodes[b] = true
	}
	for c := range nodes {
		if indeg[c] != outdeg[c] {
			return nil
		}
	}

	// Connectivity (undirected) over nodes with outgoing edges.
	und := map[byte][]byte{}
	for a, lst := range adj {
		for _, e := range lst {
			und[a] = append(und[a], e.to)
			und[e.to] = append(und[e.to], a)
		}
	}
	var active []byte
	for a, lst := range adj {
		if len(lst) > 0 {
			active = append(active, a)
		}
	}
	if len(active) == 0 {
		return nil
	}
	seen := map[byte]bool{}
	st := []byte{active[0]}
	for len(st) > 0 {
		c := st[len(st)-1]
		st = st[:len(st)-1]
		if seen[c] {
			continue
		}
		seen[c] = true
		for _, nb := range und[c] {
			if !seen[nb] {
				st = append(st, nb)
			}
		}
	}
	for _, c := range active {
		if !seen[c] {
			return nil
		}
	}

	// Hierholzer starting from first word's first char.
	start := words[0][0]
	ptr := map[byte]int{}
	nodeStack := []byte{start}
	var edgeStack, circuit []string
	for len(nodeStack) > 0 {
		v := nodeStack[len(nodeStack)-1]
		if ptr[v] < len(adj[v]) {
			e := adj[v][ptr[v]]
			ptr[v]++
			nodeStack = append(nodeStack, e.to)
			edgeStack = append(edgeStack, e.word)
		} else {
			nodeStack = nodeStack[:len(nodeStack)-1]
			if len(edgeStack) > 0 {
				circuit = append(circuit, edgeStack[len(edgeStack)-1])
				edgeStack = edgeStack[:len(edgeStack)-1]
			}
		}
	}
	if len(circuit) != len(words) {
		return nil
	}
	for i, j := 0, len(circuit)-1; i < j; i, j = i+1, j-1 {
		circuit[i], circuit[j] = circuit[j], circuit[i]
	}
	return circuit
}

func main() {
	words := []string{"chair", "height", "racket", "touch", "tunic"}
	order := circleOrder(words)
	if order != nil {
		fmt.Println(strings.Join(order, " --> ") + " --> " + order[0])
	} else {
		fmt.Println("No circle possible")
	}
}
