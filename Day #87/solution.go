// Day 87: Validate directional rules. N/S -> vertical graph, E/W -> horizontal graph,
// edge u->v means u strictly greater on that axis. A contradiction is a cycle. Time O(V+E).
package main

import "fmt"

type Graph map[string]map[string]bool

func (g Graph) edge(a, b string) {
	if g[a] == nil {
		g[a] = map[string]bool{}
	}
	if g[b] == nil {
		g[b] = map[string]bool{}
	}
	g[a][b] = true
}

func (g Graph) hasCycle() bool {
	state := map[string]int{} // 0 unvisited, 1 visiting, 2 done
	var dfs func(u string) bool
	dfs = func(u string) bool {
		state[u] = 1
		for v := range g[u] {
			if state[v] == 1 {
				return true
			}
			if state[v] == 0 && dfs(v) {
				return true
			}
		}
		state[u] = 2
		return false
	}
	for u := range g {
		if state[u] == 0 && dfs(u) {
			return true
		}
	}
	return false
}

func validate(rules [][3]string) bool {
	vert, horiz := Graph{}, Graph{}
	for _, r := range rules {
		a, dir, b := r[0], r[1], r[2]
		for _, c := range dir {
			switch c {
			case 'N':
				vert.edge(a, b)
			case 'S':
				vert.edge(b, a)
			case 'E':
				horiz.edge(a, b)
			case 'W':
				horiz.edge(b, a)
			}
		}
	}
	return !vert.hasCycle() && !horiz.hasCycle()
}

func main() {
	rules := [][3]string{{"A", "N", "B"}, {"B", "NE", "C"}, {"C", "N", "A"}}
	if validate(rules) {
		fmt.Println("validates")
	} else {
		fmt.Println("does not validate")
	}
	// does not validate
}
