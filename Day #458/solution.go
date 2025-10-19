// Day 458: Validate directional (NE/SW/...) rules for consistency.
// Decompose into x/y strict orders; a cycle in either graph = contradiction.
// Time O(R + V) via DFS cycle detection.
package main

import (
	"fmt"
	"strings"
)

type Graph struct {
	adj   map[string][]string
	nodes map[string]bool
}

func newGraph() *Graph {
	return &Graph{adj: map[string][]string{}, nodes: map[string]bool{}}
}

func (g *Graph) less(small, big string) {
	g.adj[small] = append(g.adj[small], big)
	g.nodes[small] = true
	g.nodes[big] = true
}

func (g *Graph) hasCycle() bool {
	color := map[string]int{}
	var dfs func(u string) bool
	dfs = func(u string) bool {
		color[u] = 1
		for _, v := range g.adj[u] {
			if color[v] == 1 {
				return true
			}
			if color[v] == 0 && dfs(v) {
				return true
			}
		}
		color[u] = 2
		return false
	}
	for n := range g.nodes {
		if color[n] == 0 && dfs(n) {
			return true
		}
	}
	return false
}

func validate(rules []string) bool {
	gx, gy := newGraph(), newGraph()
	for _, r := range rules {
		p := strings.Fields(r)
		a, d, b := p[0], p[1], p[2]
		for _, c := range d {
			switch c {
			case 'N':
				gy.less(b, a)
			case 'S':
				gy.less(a, b)
			case 'E':
				gx.less(b, a)
			case 'W':
				gx.less(a, b)
			}
		}
	}
	return !gx.hasCycle() && !gy.hasCycle()
}

func main() {
	fmt.Println(label(validate([]string{"A N B", "B NE C", "C N A"}))) // Invalid
	fmt.Println(label(validate([]string{"A NW B", "A N B"})))          // Valid
}

func label(ok bool) string {
	if ok {
		return "Valid"
	}
	return "Invalid"
}
