// Word-chain circle = Eulerian circuit on directed graph (node=letter, edge first->last).
// Check in==out degree for all nodes and single SCC over non-zero-degree nodes. O(V+E).
package main

import "fmt"

var indeg, outdeg [26]int
var adj, radj [26][]int
var vis [26]bool

func dfs(g *[26][]int, u int) {
	vis[u] = true
	for _, v := range g[u] {
		if !vis[v] {
			dfs(g, v)
		}
	}
}

func canChain(words []string) bool {
	for i := 0; i < 26; i++ {
		indeg[i], outdeg[i] = 0, 0
		adj[i], radj[i] = nil, nil
	}
	for _, w := range words {
		a := int(w[0] - 'a')
		b := int(w[len(w)-1] - 'a')
		outdeg[a]++
		indeg[b]++
		adj[a] = append(adj[a], b)
		radj[b] = append(radj[b], a)
	}
	for i := 0; i < 26; i++ {
		if indeg[i] != outdeg[i] {
			return false
		}
	}
	start := -1
	for i := 0; i < 26; i++ {
		if outdeg[i] > 0 {
			start = i
			break
		}
	}
	if start == -1 {
		return true
	}
	for i := range vis {
		vis[i] = false
	}
	dfs(&adj, start)
	for i := 0; i < 26; i++ {
		if (indeg[i] != 0 || outdeg[i] != 0) && !vis[i] {
			return false
		}
	}
	for i := range vis {
		vis[i] = false
	}
	dfs(&radj, start)
	for i := 0; i < 26; i++ {
		if (indeg[i] != 0 || outdeg[i] != 0) && !vis[i] {
			return false
		}
	}
	return true
}

func main() {
	words := []string{"chair", "height", "racket", "touch", "tunic"}
	if canChain(words) {
		fmt.Println("True")
	} else {
		fmt.Println("False")
	}
}
