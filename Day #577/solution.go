// Word circle: model words as directed edges first->last char; Eulerian circuit via Hierholzer. Time O(V+E), Space O(V+E).
package main

import (
	"fmt"
	"strings"
)

type edge struct {
	next byte
	word string
}

func wordCircle(words []string) []string {
	adj := map[byte][]edge{}
	for _, w := range words {
		f := w[0]
		adj[f] = append(adj[f], edge{w[len(w)-1], w})
	}
	ptr := map[byte]int{}

	start := words[0][0]
	var path []string
	type frame struct {
		ch   byte
		word string // "" means start
	}
	stack := []frame{{start, ""}}
	for len(stack) > 0 {
		v := stack[len(stack)-1].ch
		lst := adj[v]
		p := ptr[v]
		if p < len(lst) {
			ptr[v] = p + 1
			e := lst[p]
			stack = append(stack, frame{e.next, e.word})
		} else {
			top := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			if top.word != "" {
				path = append(path, top.word)
			}
		}
	}
	// reverse
	for i, j := 0, len(path)-1; i < j; i, j = i+1, j-1 {
		path[i], path[j] = path[j], path[i]
	}
	return path
}

func main() {
	words := []string{"chair", "height", "racket", "touch", "tunic"}
	path := wordCircle(words)
	fmt.Println(strings.Join(path, " --> ") + " --> " + path[0])
}
