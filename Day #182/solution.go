// Day 182: Graph is minimally-connected iff it is a tree (connected and |E| == |V|-1).
// BFS connectivity + edge count. Time O(V+E), Space O(V+E).
package main

import "fmt"

func isMinimallyConnected(v int, edges [][2]int) bool {
	if len(edges) != v-1 {
		return false
	}
	adj := make([][]int, v)
	for _, e := range edges {
		adj[e[0]] = append(adj[e[0]], e[1])
		adj[e[1]] = append(adj[e[1]], e[0])
	}
	seen := make([]bool, v)
	queue := []int{0}
	seen[0] = true
	cnt := 1
	for len(queue) > 0 {
		u := queue[0]
		queue = queue[1:]
		for _, w := range adj[u] {
			if !seen[w] {
				seen[w] = true
				cnt++
				queue = append(queue, w)
			}
		}
	}
	return cnt == v
}

func main() {
	tree := [][2]int{{0, 1}, {0, 2}, {1, 3}, {1, 4}}
	cyclic := [][2]int{{0, 1}, {0, 2}, {1, 3}, {1, 4}, {3, 4}}
	fmt.Println(isMinimallyConnected(5, tree))
	fmt.Println(isMinimallyConnected(5, cyclic))
}
