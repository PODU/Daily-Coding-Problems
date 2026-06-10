// Word ladder: BFS over dictionary words (one-letter changes), tracking parents to rebuild shortest path. O(N*L*N) time.
package main

import (
	"fmt"
	"strings"
)

func differsByOne(a, b string) bool {
	if len(a) != len(b) {
		return false
	}
	diff := 0
	for i := 0; i < len(a); i++ {
		if a[i] != b[i] {
			diff++
			if diff > 1 {
				return false
			}
		}
	}
	return diff == 1
}

// returns nil if no path
func ladder(start, end string, dict []string) []string {
	visited := map[string]bool{start: true}
	q := []string{start}
	parent := map[string]string{}
	for len(q) > 0 {
		cur := q[0]
		q = q[1:]
		if cur == end {
			var path []string
			at := cur
			for {
				path = append(path, at)
				if at == start {
					break
				}
				at = parent[at]
			}
			for i, j := 0, len(path)-1; i < j; i, j = i+1, j-1 {
				path[i], path[j] = path[j], path[i]
			}
			return path
		}
		for _, w := range dict {
			if !visited[w] && differsByOne(cur, w) {
				visited[w] = true
				parent[w] = cur
				q = append(q, w)
			}
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
	fmt.Println(format(ladder("dog", "cat", []string{"tod", "dat", "dar", "dot"})))
}
