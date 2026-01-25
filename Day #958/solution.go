// Day 958: reconstruct itinerary using every flight once, lexicographically smallest.
// Backtracking DFS over sorted adjacency. Worst O(E!) but fast in practice; Space O(E).
package main

import (
	"fmt"
	"sort"
	"strings"
)

func itinerary(flights [][2]string, start string) []string {
	adj := map[string][]string{}
	used := map[string][]bool{}
	for _, f := range flights {
		adj[f[0]] = append(adj[f[0]], f[1])
	}
	for k := range adj {
		sort.Strings(adj[k])
		used[k] = make([]bool, len(adj[k]))
	}
	total := len(flights)
	path := []string{start}

	var dfs func(node string) bool
	dfs = func(node string) bool {
		if len(path) == total+1 {
			return true
		}
		dests := adj[node]
		for i := range dests {
			if used[node][i] {
				continue
			}
			used[node][i] = true
			path = append(path, dests[i])
			if dfs(dests[i]) {
				return true
			}
			path = path[:len(path)-1]
			used[node][i] = false
		}
		return false
	}

	if dfs(start) {
		return path
	}
	return nil
}

func show(v []string) string {
	if v == nil {
		return "null"
	}
	parts := make([]string, len(v))
	for i, x := range v {
		parts[i] = "'" + x + "'"
	}
	return "[" + strings.Join(parts, ", ") + "]"
}

func main() {
	fmt.Println(show(itinerary([][2]string{{"SFO", "HKO"}, {"YYZ", "SFO"}, {"YUL", "YYZ"}, {"HKO", "ORD"}}, "YUL")))
	fmt.Println(show(itinerary([][2]string{{"SFO", "COM"}, {"COM", "YYZ"}}, "COM")))
	fmt.Println(show(itinerary([][2]string{{"A", "B"}, {"A", "C"}, {"B", "C"}, {"C", "A"}}, "A")))
}
