// Day 1031: Snakes & Ladders min turns. BFS over squares 1..100, each turn rolls 1..6,
// applying snake/ladder mapping when a roll lands on a key. Time O(100*6), Space O(100).
package main

import "fmt"

func minTurns() int {
	jumps := map[int]int{
		16: 6, 48: 26, 49: 11, 56: 53, 62: 19, 64: 60, 87: 24, 93: 73, 95: 75, 98: 78,
		1: 38, 4: 14, 9: 31, 21: 42, 28: 84, 36: 44, 51: 67, 71: 91, 80: 100,
	}
	dist := make([]int, 101)
	for i := range dist {
		dist[i] = -1
	}
	dist[1] = 0
	q := []int{1}
	for len(q) > 0 {
		s := q[0]
		q = q[1:]
		if s == 100 {
			return dist[s]
		}
		for d := 1; d <= 6; d++ {
			nx := s + d
			if nx > 100 {
				continue
			}
			if v, ok := jumps[nx]; ok {
				nx = v
			}
			if dist[nx] == -1 {
				dist[nx] = dist[s] + 1
				q = append(q, nx)
			}
		}
	}
	return -1
}

func main() {
	fmt.Println(minTurns())
}
