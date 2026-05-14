// Minimally-connected = tree: edges == n-1 AND connected. Use BFS.
// Time O(n + e), Space O(n).
package main

import "fmt"

func isTree(n int, edges [][2]int) bool {
	if len(edges) != n-1 {
		return false
	}
	adj := make([][]int, n)
	for _, e := range edges {
		adj[e[0]] = append(adj[e[0]], e[1])
		adj[e[1]] = append(adj[e[1]], e[0])
	}
	visited := make([]bool, n)
	queue := []int{0}
	visited[0] = true
	count := 1
	for len(queue) > 0 {
		u := queue[0]
		queue = queue[1:]
		for _, v := range adj[u] {
			if !visited[v] {
				visited[v] = true
				count++
				queue = append(queue, v)
			}
		}
	}
	return count == n
}

func main() {
	n := 4
	edges := [][2]int{{0, 1}, {1, 2}, {1, 3}}
	if isTree(n, edges) {
		fmt.Println("True")
	} else {
		fmt.Println("False")
	}
}
