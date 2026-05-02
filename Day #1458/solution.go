// Snakes & Ladders: BFS over squares 1..100, dice rolls 1..6, apply jumps. Min turns 1->100.
// Time: O(100*6). Space: O(100).
package main

import "fmt"

func minTurns() int {
	jump := map[int]int{
		16: 6, 48: 26, 49: 11, 56: 53, 62: 19, 64: 60, 87: 24, 93: 73, 95: 75, 98: 78, // snakes
		1: 38, 4: 14, 9: 31, 21: 42, 28: 84, 36: 44, 51: 67, 71: 91, 80: 100, // ladders
	}
	land := func(s int) int {
		if v, ok := jump[s]; ok {
			return v
		}
		return s
	}

	start := land(1)
	dist := make([]int, 101)
	for i := range dist {
		dist[i] = -1
	}
	dist[start] = 0
	queue := []int{start}
	for len(queue) > 0 {
		s := queue[0]
		queue = queue[1:]
		if s == 100 {
			return dist[s]
		}
		for d := 1; d <= 6; d++ {
			nxt := s + d
			if nxt > 100 {
				continue
			}
			nxt = land(nxt)
			if dist[nxt] == -1 {
				dist[nxt] = dist[s] + 1
				queue = append(queue, nxt)
			}
		}
	}
	return -1
}

func main() {
	fmt.Println(minTurns())
}
