// Day 1053: Directional consistency. Decompose each rule into strict x/y
// inequalities, build a directed "greater-than" graph per axis, and detect a
// cycle (contradiction) via DFS. Time O(V+E), Space O(V+E).

package main

import (
	"fmt"
	"strings"
)

type Graph map[string]map[string]bool

func add(g Graph, u, v string) {
	if g[u] == nil {
		g[u] = map[string]bool{}
	}
	if g[v] == nil {
		g[v] = map[string]bool{}
	}
	g[u][v] = true
}

func dfs(g Graph, u string, state map[string]int) bool {
	state[u] = 0
	for w := range g[u] {
		if s, ok := state[w]; ok && s == 0 {
			return true
		}
		if _, ok := state[w]; !ok && dfs(g, w, state) {
			return true
		}
	}
	state[u] = 1
	return false
}

func hasCycle(g Graph) bool {
	state := map[string]int{}
	for n := range g {
		if _, ok := state[n]; !ok && dfs(g, n, state) {
			return true
		}
	}
	return false
}

func validate(rules []string) bool {
	gx, gy := Graph{}, Graph{}
	for _, rule := range rules {
		p := strings.Fields(rule)
		a, d, b := p[0], p[1], p[2]
		for _, ch := range d {
			switch ch {
			case 'N':
				add(gy, a, b)
			case 'S':
				add(gy, b, a)
			case 'E':
				add(gx, a, b)
			case 'W':
				add(gx, b, a)
			}
		}
	}
	return !(hasCycle(gx) || hasCycle(gy))
}

func main() {
	ex1 := []string{"A N B", "B NE C", "C N A"}
	ex2 := []string{"A NW B", "A N B"}
	if !validate(ex1) {
		fmt.Println("does not validate, since A cannot be both north and south of C.")
	} else {
		fmt.Println("is considered valid.")
	}
	if validate(ex2) {
		fmt.Println("is considered valid.")
	} else {
		fmt.Println("does not validate.")
	}
}
