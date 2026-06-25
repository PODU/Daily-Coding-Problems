// Transitive closure via DFS from each vertex. Time O(V*(V+E)), Space O(V^2).
package main

import (
	"fmt"
	"strings"
)

func main() {
	graph := [][]int{{0, 1, 3}, {1, 2}, {2}, {3}}
	n := len(graph)
	M := make([][]int, n)
	for i := range M {
		M[i] = make([]int, n)
	}
	for s := 0; s < n; s++ {
		vis := make([]bool, n)
		stack := []int{s}
		for len(stack) > 0 {
			u := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			if vis[u] {
				continue
			}
			vis[u] = true
			M[s][u] = 1
			for _, v := range graph[u] {
				if !vis[v] {
					stack = append(stack, v)
				}
			}
		}
	}
	for i := 0; i < n; i++ {
		parts := make([]string, n)
		for j := 0; j < n; j++ {
			parts[j] = fmt.Sprintf("%d", M[i][j])
		}
		fmt.Println("[" + strings.Join(parts, ", ") + "]")
	}
}
