// Day 1454: A graph is minimally-connected iff it is a tree: connected AND has
// no cycle (exactly n-1 edges). DFS from node 0. Time O(V+E), Space O(V+E).
package main

import "fmt"

func isTree(n int, edges [][2]int) bool {
	if n == 0 {
		return true
	}
	adj := make([][]int, n)
	for _, e := range edges {
		adj[e[0]] = append(adj[e[0]], e[1])
		adj[e[1]] = append(adj[e[1]], e[0])
	}
	visited := make([]bool, n)
	visited[0] = true
	seen := 1
	type frame struct{ u, parent int }
	stack := []frame{{0, -1}}
	for len(stack) > 0 {
		f := stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		for _, w := range adj[f.u] {
			if !visited[w] {
				visited[w] = true
				seen++
				stack = append(stack, frame{w, f.u})
			} else if w != f.parent {
				return false // back-edge -> cycle
			}
		}
	}
	return seen == n
}

func main() {
	tree := [][2]int{{0, 1}, {1, 2}, {1, 3}}
	cyclic := [][2]int{{0, 1}, {1, 2}, {2, 0}, {2, 3}}
	pr := func(b bool) {
		if b {
			fmt.Println("True")
		} else {
			fmt.Println("False")
		}
	}
	pr(isTree(4, tree))   // True
	pr(isTree(4, cyclic)) // False
}
