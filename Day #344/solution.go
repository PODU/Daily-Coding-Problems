// Max edges removed so every component has even node count. DFS subtree sizes;
// answer = count of non-root nodes with even subtree size. Time O(n), Space O(n).
// Note: README narrates one valid single cut (3,4), but the MAXIMUM is 2:
// cut (1,3) and (3,4) -> {1,2},{3,5},{4,6,7,8}, all even.
package main

import "fmt"

var adj [][]int
var answer int

func dfs(u, parent, root int) int {
	s := 1
	for _, v := range adj[u] {
		if v != parent {
			s += dfs(v, u, root)
		}
	}
	if u != root && s%2 == 0 {
		answer++
	}
	return s
}

func main() {
	n := 8
	adj = make([][]int, n+1)
	edges := [][2]int{{1, 2}, {1, 3}, {3, 4}, {3, 5}, {4, 6}, {4, 7}, {4, 8}}
	for _, e := range edges {
		adj[e[0]] = append(adj[e[0]], e[1])
		adj[e[1]] = append(adj[e[1]], e[0])
	}
	dfs(1, 0, 1)
	fmt.Println(answer)
}
