// Transitive closure: each input row is [node, neighbors...]. DFS from every node
// marks all reachable vertices (incl. itself). Time O(V*(V+E)), Space O(V^2).
package main

import "fmt"

func transitiveClosure(graph [][]int) [][]int {
	n := len(graph)
	adj := make([][]int, n)
	for _, r := range graph {
		node := r[0]
		adj[node] = append(adj[node], r[1:]...)
	}
	M := make([][]int, n)
	for i := range M {
		M[i] = make([]int, n)
	}
	var dfs func(u int, row []int)
	dfs = func(u int, row []int) {
		row[u] = 1
		for _, v := range adj[u] {
			if row[v] == 0 {
				dfs(v, row)
			}
		}
	}
	for i := 0; i < n; i++ {
		dfs(i, M[i])
	}
	return M
}

func main() {
	graph := [][]int{{0, 1, 3}, {1, 2}, {2}, {3}}
	for _, row := range transitiveClosure(graph) {
		fmt.Println(row)
	}
}
