// Count connected components (friend groups) in an undirected graph via DFS.
// Time O(V+E), Space O(V).
package main

import (
	"fmt"
	"sort"
)

func main() {
	adj := map[int][]int{
		0: {1, 2},
		1: {0, 5},
		2: {0},
		3: {6},
		4: {},
		5: {1},
		6: {3},
	}

	keys := make([]int, 0, len(adj))
	for k := range adj {
		keys = append(keys, k)
	}
	sort.Ints(keys)

	visited := make(map[int]bool)
	groups := 0
	for _, start := range keys {
		if visited[start] {
			continue
		}
		groups++
		stack := []int{start}
		visited[start] = true
		for len(stack) > 0 {
			u := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			for _, v := range adj[u] {
				if !visited[v] {
					visited[v] = true
					stack = append(stack, v)
				}
			}
		}
	}
	fmt.Println(groups)
}
