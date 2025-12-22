// Day 780: Topological sort of courses (prereqs map). DFS post-order with
// cycle detection; returns nil if a cycle exists. O(V + E).
package main

import (
	"fmt"
	"sort"
	"strings"
)

func dfs(c string, g map[string][]string, state map[string]int, order *[]string) bool {
	state[c] = 1
	for _, pre := range g[c] {
		if state[pre] == 1 {
			return false
		}
		if state[pre] == 0 && !dfs(pre, g, state, order) {
			return false
		}
	}
	state[c] = 2
	*order = append(*order, c)
	return true
}

func courseOrder(g map[string][]string) []string {
	keys := make([]string, 0, len(g))
	for k := range g {
		keys = append(keys, k)
	}
	sort.Strings(keys)
	state := map[string]int{}
	order := []string{}
	for _, c := range keys {
		if state[c] == 0 && !dfs(c, g, state, &order) {
			return nil
		}
	}
	return order
}

func main() {
	g := map[string][]string{
		"CSC300": {"CSC100", "CSC200"},
		"CSC200": {"CSC100"},
		"CSC100": {},
	}
	r := courseOrder(g)
	if r == nil {
		fmt.Println("null")
		return
	}
	fmt.Println("['" + strings.Join(r, "', '") + "']") // ['CSC100', 'CSC200', 'CSC300']
}
