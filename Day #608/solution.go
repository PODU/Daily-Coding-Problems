// Word Ladder: BFS over words, mutating one letter at a time; track parents to rebuild path.
// Time: O(N * L * 26). Space: O(N * L).
package main

import (
	"fmt"
	"strings"
)

func wordLadder(start, end string, dictionary []string) []string {
	dict := make(map[string]bool)
	for _, w := range dictionary {
		dict[w] = true
	}
	if !dict[end] {
		return nil
	}
	parent := map[string]string{start: ""}
	visited := map[string]bool{start: true}
	queue := []string{start}
	for len(queue) > 0 {
		cur := queue[0]
		queue = queue[1:]
		if cur == end {
			var path []string
			for w := end; ; w = parent[w] {
				path = append([]string{w}, path...)
				if w == start {
					break
				}
			}
			return path
		}
		b := []byte(cur)
		for i := 0; i < len(b); i++ {
			orig := b[i]
			for c := byte('a'); c <= 'z'; c++ {
				if c == orig {
					continue
				}
				b[i] = c
				nxt := string(b)
				if dict[nxt] && !visited[nxt] {
					visited[nxt] = true
					parent[nxt] = cur
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
	parts := make([]string, len(path))
	for i, w := range path {
		parts[i] = "\"" + w + "\""
	}
	return "[" + strings.Join(parts, ", ") + "]"
}

func main() {
	fmt.Println(format(wordLadder("dog", "cat", []string{"dot", "dop", "dat", "cat"})))
	fmt.Println(format(wordLadder("dog", "cat", []string{"dot", "tod", "dat", "dar"})))
}
