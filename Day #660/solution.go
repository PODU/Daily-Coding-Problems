// Alien dictionary: build edges from first differing char of adjacent words,
// then Kahn's BFS topological sort. Time O(C + V + E), Space O(V + E).
package main

import (
	"fmt"
	"sort"
	"strings"
)

func alienOrder(words []string) []rune {
	graph := map[rune]map[rune]bool{}
	indeg := map[rune]int{}
	for _, w := range words {
		for _, c := range w {
			if graph[c] == nil {
				graph[c] = map[rune]bool{}
				indeg[c] = 0
			}
		}
	}
	for i := 0; i+1 < len(words); i++ {
		a, b := []rune(words[i]), []rune(words[i+1])
		n := len(a)
		if len(b) < n {
			n = len(b)
		}
		for j := 0; j < n; j++ {
			if a[j] != b[j] {
				if !graph[a[j]][b[j]] {
					graph[a[j]][b[j]] = true
					indeg[b[j]]++
				}
				break
			}
		}
	}
	var queue []rune
	for c, d := range indeg {
		if d == 0 {
			queue = append(queue, c)
		}
	}
	sort.Slice(queue, func(i, j int) bool { return queue[i] < queue[j] })
	var order []rune
	for len(queue) > 0 {
		c := queue[0]
		queue = queue[1:]
		order = append(order, c)
		var nexts []rune
		for nxt := range graph[c] {
			nexts = append(nexts, nxt)
		}
		sort.Slice(nexts, func(i, j int) bool { return nexts[i] < nexts[j] })
		for _, nxt := range nexts {
			indeg[nxt]--
			if indeg[nxt] == 0 {
				queue = append(queue, nxt)
			}
		}
		sort.Slice(queue, func(i, j int) bool { return queue[i] < queue[j] })
	}
	return order
}

func main() {
	words := []string{"xww", "wxyz", "wxyw", "ywx", "ywz"}
	order := alienOrder(words)
	parts := make([]string, len(order))
	for i, c := range order {
		parts[i] = "'" + string(c) + "'"
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
