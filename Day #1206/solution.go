// Day 1206: Validate directional (N/S/E/W) rules for consistency.
// Split into vertical & horizontal strict-order graphs; a cycle = contradiction. Time O(V+E), Space O(V+E).
package main

import (
	"fmt"
	"strings"
)

type graph map[string][]string

func add(g graph, u, v string) {
	if _, ok := g[u]; !ok {
		g[u] = nil
	}
	if _, ok := g[v]; !ok {
		g[v] = nil
	}
	g[u] = append(g[u], v)
}

func hasCycle(g graph) bool {
	color := map[string]int{}
	var dfs func(u string) bool
	dfs = func(u string) bool {
		color[u] = 1
		for _, v := range g[u] {
			if color[v] == 1 {
				return true
			}
			if color[v] == 0 && dfs(v) {
				return true
			}
		}
		color[u] = 2
		return false
	}
	for u := range g {
		if color[u] == 0 && dfs(u) {
			return true
		}
	}
	return false
}

func validate(rules [][3]string) bool {
	gy, gx := graph{}, graph{}
	for _, r := range rules {
		a, d, b := r[0], r[1], r[2]
		if strings.Contains(d, "N") {
			add(gy, a, b)
		}
		if strings.Contains(d, "S") {
			add(gy, b, a)
		}
		if strings.Contains(d, "E") {
			add(gx, a, b)
		}
		if strings.Contains(d, "W") {
			add(gx, b, a)
		}
	}
	return !hasCycle(gy) && !hasCycle(gx)
}

func main() {
	rules := [][3]string{{"A", "N", "B"}, {"B", "NE", "C"}, {"C", "N", "A"}}
	if validate(rules) {
		fmt.Println("validates")
	} else {
		fmt.Println("does not validate") // expected
	}
}
