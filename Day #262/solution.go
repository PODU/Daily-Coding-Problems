// Day 262: Find all bridges in a connected undirected graph.
// Approach: Tarjan's bridge-finding algorithm using DFS with disc/low timestamps.
// An edge (u,v) is a bridge iff low[v] > disc[u]. Time O(V+E), Space O(V+E).

package main

import (
	"fmt"
	"sort"
	"strings"
)

type bridgeFinder struct {
	n       int
	adj     [][]int
	timer   int
	disc    []int
	low     []int
	bridges [][2]int
}

func newBridgeFinder(n int) *bridgeFinder {
	return &bridgeFinder{n: n, adj: make([][]int, n), disc: make([]int, n), low: make([]int, n)}
}

func (g *bridgeFinder) addEdge(u, v int) {
	g.adj[u] = append(g.adj[u], v)
	g.adj[v] = append(g.adj[v], u)
}

func (g *bridgeFinder) dfs(u, parent int) {
	g.timer++
	g.disc[u] = g.timer
	g.low[u] = g.timer
	skippedParent := false
	for _, v := range g.adj[u] {
		if v == parent && !skippedParent {
			skippedParent = true
			continue
		}
		if g.disc[v] == 0 {
			g.dfs(v, u)
			if g.low[v] < g.low[u] {
				g.low[u] = g.low[v]
			}
			if g.low[v] > g.disc[u] {
				a, b := u, v
				if a > b {
					a, b = b, a
				}
				g.bridges = append(g.bridges, [2]int{a, b})
			}
		} else if g.disc[v] < g.low[u] {
			g.low[u] = g.disc[v]
		}
	}
}

func (g *bridgeFinder) findBridges() [][2]int {
	for i := 0; i < g.n; i++ {
		if g.disc[i] == 0 {
			g.dfs(i, -1)
		}
	}
	sort.Slice(g.bridges, func(i, j int) bool {
		if g.bridges[i][0] != g.bridges[j][0] {
			return g.bridges[i][0] < g.bridges[j][0]
		}
		return g.bridges[i][1] < g.bridges[j][1]
	})
	return g.bridges
}

func main() {
	g := newBridgeFinder(5)
	edges := [][2]int{{0, 1}, {1, 2}, {2, 0}, {1, 3}, {3, 4}}
	for _, e := range edges {
		g.addEdge(e[0], e[1])
	}
	bridges := g.findBridges()
	parts := make([]string, len(bridges))
	for i, b := range bridges {
		parts[i] = fmt.Sprintf("(%d, %d)", b[0], b[1])
	}
	fmt.Println("Bridges: [" + strings.Join(parts, ", ") + "]")
}
