// BFS over squares 1..100; from s try rolls 1..6, apply snake/ladder mapping. Time O(N*6), Space O(N).
package main

import "fmt"

func minTurns() int {
	jump := map[int]int{
		16: 6, 48: 26, 49: 11, 56: 53, 62: 19, 64: 60, 87: 24, 93: 73, 95: 75, 98: 78, // snakes
		1: 38, 4: 14, 9: 31, 21: 42, 28: 84, 36: 44, 51: 67, 71: 91, 80: 100, // ladders
	}
	dist := make([]int, 101)
	for i := range dist {
		dist[i] = -1
	}
	dist[1] = 0 // start placed on 1; do NOT apply 1->38 here
	queue := []int{1}
	for len(queue) > 0 {
		s := queue[0]
		queue = queue[1:]
		if s == 100 {
			return dist[s]
		}
		for r := 1; r <= 6; r++ {
			nxt := s + r
			if nxt > 100 {
				continue
			}
			if d, ok := jump[nxt]; ok {
				nxt = d
			}
			if dist[nxt] == -1 {
				dist[nxt] = dist[s] + 1
				queue = append(queue, nxt)
			}
		}
	}
	return dist[100]
}

func main() {
	fmt.Printf("Minimum turns: %d\n", minTurns())
}
