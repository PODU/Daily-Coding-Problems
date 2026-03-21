// Word ladder: BFS over words differing by one letter. Time O(N*L*26).
package main

import "fmt"

func wordLadder(start, end string, dictionary []string) []string {
	dict := make(map[string]bool)
	for _, w := range dictionary {
		dict[w] = true
	}
	if !dict[end] {
		return nil
	}
	queue := [][]string{{start}}
	seen := map[string]bool{start: true}
	for len(queue) > 0 {
		path := queue[0]
		queue = queue[1:]
		word := path[len(path)-1]
		if word == end {
			return path
		}
		b := []byte(word)
		for i := 0; i < len(b); i++ {
			orig := b[i]
			for c := byte('a'); c <= 'z'; c++ {
				b[i] = c
				nxt := string(b)
				if dict[nxt] && !seen[nxt] {
					seen[nxt] = true
					np := make([]string, len(path))
					copy(np, path)
					queue = append(queue, append(np, nxt))
				}
			}
			b[i] = orig
		}
	}
	return nil
}

func main() {
	fmt.Println(wordLadder("dog", "cat", []string{"dot", "dop", "dat", "cat"}))
	r := wordLadder("dog", "cat", []string{"dot", "tod", "dat", "dar"})
	if r == nil {
		fmt.Println("null")
	} else {
		fmt.Println(r)
	}
}
