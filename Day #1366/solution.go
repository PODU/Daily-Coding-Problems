// Bipartite check via BFS 2-coloring. Time O(V+E), Space O(V).
package main

import "fmt"

func isBipartite(n int, adj [][]int) bool {
	color := make([]int, n)
	for i := range color {
		color[i] = -1
	}
	for s := 0; s < n; s++ {
		if color[s] != -1 {
			continue
		}
		q := []int{s}
		color[s] = 0
		for len(q) > 0 {
			u := q[0]
			q = q[1:]
			for _, v := range adj[u] {
				if color[v] == -1 {
					color[v] = color[u] ^ 1
					q = append(q, v)
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
	fmt.Println(isBipartite(4, build(4, [][2]int{{0, 1}, {1, 2}, {2, 3}, {3, 0}}))) // true
	fmt.Println(isBipartite(3, build(3, [][2]int{{0, 1}, {1, 2}, {2, 0}})))         // false
}
