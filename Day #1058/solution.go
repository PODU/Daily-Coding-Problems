// Bipartite check via BFS 2-coloring over all components.
// Time: O(V + E), Space: O(V).
package main

import "fmt"

func isBipartite(adj [][]int) bool {
	n := len(adj)
	color := make([]int, n)
	for i := range color {
		color[i] = -1
	}
	for s := 0; s < n; s++ {
		if color[s] != -1 {
			continue
		}
		color[s] = 0
		queue := []int{s}
		for len(queue) > 0 {
			u := queue[0]
			queue = queue[1:]
			for _, v := range adj[u] {
				if color[v] == -1 {
					color[v] = color[u] ^ 1
					queue = append(queue, v)
				} else if color[v] == color[u] {
					return false
				}
			}
		}
	}
	return true
}

func build(n int, edges [][2]int) [][]int {
	adj := make([][]int, n)
	for _, e := range edges {
		adj[e[0]] = append(adj[e[0]], e[1])
		adj[e[1]] = append(adj[e[1]], e[0])
	}
	return adj
}

func main() {
	g1 := build(4, [][2]int{{0, 1}, {1, 2}, {2, 3}, {3, 0}}) // 4-cycle -> true
	g2 := build(3, [][2]int{{0, 1}, {1, 2}, {2, 0}})         // triangle -> false
	fmt.Println(isBipartite(g1))
	fmt.Println(isBipartite(g2))
}
