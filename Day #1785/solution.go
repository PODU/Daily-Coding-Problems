// Build origin->sorted destinations; DFS backtracking trying lexicographically
// smallest dest first; first path using all flights is the answer.
// Time O(E!) worst case, Space O(E).
package main

import (
	"fmt"
	"sort"
	"strings"
)

func findItinerary(flights [][2]string, start string) []string {
	total := len(flights)
	graph := map[string][]string{}
	for _, f := range flights {
		graph[f[0]] = append(graph[f[0]], f[1])
	}
	for k := range graph {
		sort.Strings(graph[k])
	}

	var dfs func(node string, path []string) []string
	dfs = func(node string, path []string) []string {
		if len(path) == total+1 {
			cp := make([]string, len(path))
			copy(cp, path)
			return cp
		}
		dests := graph[node]
		for i := 0; i < len(dests); i++ {
			if dests[i] == "" {
				continue
			}
			d := dests[i]
			dests[i] = ""
			res := dfs(d, append(path, d))
			if res != nil {
				return res
			}
			dests[i] = d
		}
		return nil
	}
	return dfs(start, []string{start})
}

func show(r []string) {
	if r == nil {
		fmt.Println("null")
		return
	}
	parts := make([]string, len(r))
	for i, s := range r {
		parts[i] = "'" + s + "'"
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}

func main() {
	show(findItinerary([][2]string{{"SFO", "HKO"}, {"YYZ", "SFO"}, {"YUL", "YYZ"}, {"HKO", "ORD"}}, "YUL"))
	show(findItinerary([][2]string{{"SFO", "COM"}, {"COM", "YYZ"}}, "COM"))
	show(findItinerary([][2]string{{"A", "B"}, {"A", "C"}, {"B", "C"}, {"C", "A"}}, "A"))
}
