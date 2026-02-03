// Word ladder: BFS over words, change one letter per step, track predecessors.
// Time: O(N * L * 26), Space: O(N). Returns shortest path or nil (null).
package main

import (
	"fmt"
	"strings"
)

func ladder(start, end string, dict []string) []string {
	if start == end {
		return []string{start}
	}
	words := make(map[string]bool)
	for _, w := range dict {
		words[w] = true
	}
	queue := []string{start}
	parent := map[string]string{start: ""}
	visited := map[string]bool{start: true}

	for len(queue) > 0 {
		cur := queue[0]
		queue = queue[1:]
		b := []byte(cur)
		for i := 0; i < len(b); i++ {
			orig := b[i]
			for c := byte('a'); c <= 'z'; c++ {
				if c == orig {
					continue
				}
				b[i] = c
				nxt := string(b)
				if words[nxt] && !visited[nxt] {
					visited[nxt] = true
					parent[nxt] = cur
					if nxt == end {
						var path []string
						for w := end; w != ""; w = parent[w] {
							path = append([]string{w}, path...)
						}
						return path
					}
					queue = append(queue, nxt)
				}
			}
			b[i] = orig
		}
	}
	return nil // no path
}

func printPath(p []string) {
	if p == nil {
		fmt.Println("null")
		return
	}
	quoted := make([]string, len(p))
	for i, w := range p {
		quoted[i] = "\"" + w + "\""
	}
	fmt.Println("[" + strings.Join(quoted, ", ") + "]")
}

func main() {
	printPath(ladder("dog", "cat", []string{"dot", "dop", "dat", "cat"})) // ["dog", "dot", "dat", "cat"]
	printPath(ladder("dog", "cat", []string{"dot", "tod", "dat", "dar"})) // null
}
