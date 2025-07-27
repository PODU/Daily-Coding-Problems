// Reconstruct Itinerary: backtracking DFS over lexicographically sorted edges.
// First complete itinerary using all edges (tried in lex order) is the answer.
// Time: exponential worst case; Space: O(E).
package main

import (
	"fmt"
	"sort"
	"strings"
)

var adj map[string][]string
var used map[string][]bool
var totalEdges int
var path []string

func dfs(node string) bool {
	if len(path) == totalEdges+1 {
		return true
	}
	dests, ok := adj[node]
	if !ok {
		return false
	}
	u := used[node]
	for i := 0; i < len(dests); i++ {
		if u[i] {
			continue
		}
		u[i] = true
		path = append(path, dests[i])
		if dfs(dests[i]) {
			return true
		}
		path = path[:len(path)-1]
		u[i] = false
	}
	return false
}

func reconstruct(flights [][2]string, start string) []string {
	adj = map[string][]string{}
	used = map[string][]bool{}
	for _, f := range flights {
		adj[f[0]] = append(adj[f[0]], f[1])
	}
	for k := range adj {
		sort.Strings(adj[k])
		used[k] = make([]bool, len(adj[k]))
	}
	totalEdges = len(flights)
	path = []string{start}
	if dfs(start) {
		return path
	}
	return nil
}

func fmtList(v []string) string {
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
	fmt.Println(fmtList(reconstruct([][2]string{{"SFO", "HKO"}, {"YYZ", "SFO"}, {"YUL", "YYZ"}, {"HKO", "ORD"}}, "YUL")))
	fmt.Println(fmtList(reconstruct([][2]string{{"SFO", "COM"}, {"COM", "YYZ"}}, "COM")))
	fmt.Println(fmtList(reconstruct([][2]string{{"A", "B"}, {"A", "C"}, {"B", "C"}, {"C", "A"}}, "A")))
}
