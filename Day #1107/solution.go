// Day 1107: Max edges removable so every component has an even node count.
// DFS subtree sizes; every non-root node with an even-sized subtree => one cuttable edge.
// Time: O(V+E). Space: O(V).
package main

import "fmt"

func maxEdgesRemoved(n int, edges [][2]int, root int) int {
	adj := make([][]int, n+1)
	for _, e := range edges {
		adj[e[0]] = append(adj[e[0]], e[1])
		adj[e[1]] = append(adj[e[1]], e[0])
	}
	answer := 0
	var dfs func(u, parent int) int
	dfs = func(u, parent int) int {
		size := 1
		for _, v := range adj[u] {
			if v != parent {
				size += dfs(v, u)
			}
		}
		if parent != -1 && size%2 == 0 {
			answer++
		}
		return size
	}
	dfs(root, -1)
	return answer
}

func main() {
	edges := [][2]int{{1, 2}, {1, 3}, {3, 4}, {3, 5}, {4, 6}, {4, 7}, {4, 8}}
	fmt.Println(maxEdgesRemoved(8, edges, 1)) // 2
}
