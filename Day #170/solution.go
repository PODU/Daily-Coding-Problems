// Word ladder via BFS over one-letter transformations. O(N * L * 26) time, O(N) space.
package main

import (
	"fmt"
	"strings"
)

func ladder(start, end string, dictionary []string) []string {
	if start == end {
		return []string{start}
	}
	dict := make(map[string]bool)
	for _, w := range dictionary {
		dict[w] = true
	}
	queue := []string{start}
	parent := map[string]string{start: ""}
	seen := map[string]bool{start: true}
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
				if dict[nxt] && !seen[nxt] {
					seen[nxt] = true
					parent[nxt] = cur
					if nxt == end {
						var path []string
						for s := end; s != ""; s = parent[s] {
							path = append([]string{s}, path...)
						}
						return path
					}
					queue = append(queue, nxt)
				}
			}
			b[i] = orig
		}
	}
	return nil
}

func format(path []string) string {
	if path == nil {
		return "null"
	}
	quoted := make([]string, len(path))
	for i, w := range path {
		quoted[i] = "\"" + w + "\""
	}
	return "[" + strings.Join(quoted, ", ") + "]"
}

func main() {
	fmt.Println(format(ladder("dog", "cat", []string{"dot", "dop", "dat", "cat"})))
	fmt.Println(format(ladder("dog", "cat", []string{"dot", "tod", "dat", "dar"})))
}
