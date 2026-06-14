// DFS subtree sizes; count non-root subtrees with even size = max edges removable.
// Time O(n), Space O(n).
package main

import "fmt"

var g [][]int
var ans int

func dfs(u, p int) int {
	sz := 1
	for _, v := range g[u] {
		if v != p {
			sz += dfs(v, u)
		}
	}
	if p != -1 && sz%2 == 0 {
		ans++
	}
	return sz
}

func main() {
	n := 8
	g = make([][]int, n+1)
	edges := [][2]int{{1, 2}, {1, 3}, {3, 4}, {3, 5}, {4, 6}, {4, 7}, {4, 8}}
	for _, e := range edges {
		g[e[0]] = append(g[e[0]], e[1])
		g[e[1]] = append(g[e[1]], e[0])
	}
	dfs(1, -1)
	fmt.Println(ans)
}
