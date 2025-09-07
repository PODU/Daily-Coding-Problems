// Snakes and Ladders: BFS over squares 1..100, each die roll (1..6) is one edge; apply jumps.
// Time: O(100 * 6), Space: O(100).
package main

import "fmt"

func minTurns() int {
	jump := map[int]int{
		16: 6, 48: 26, 49: 11, 56: 53, 62: 19, 64: 60, 87: 24, 93: 73, 95: 75, 98: 78,
		1: 38, 4: 14, 9: 31, 21: 42, 28: 84, 36: 44, 51: 67, 71: 91, 80: 100}
	apply := func(p int) int {
		if v, ok := jump[p]; ok {
			return v
		}
		return p
	}
	dist := make([]int, 101)
	for i := range dist {
		dist[i] = -1
	}
	start := apply(1)
	q := []int{start}
	dist[start] = 0
	for len(q) > 0 {
		p := q[0]
		q = q[1:]
		if p == 100 {
			return dist[p]
		}
		for d := 1; d <= 6; d++ {
			np := p + d
			if np > 100 {
				continue
			}
			np = apply(np)
			if dist[np] == -1 {
				dist[np] = dist[p] + 1
				q = append(q, np)
			}
		}
	}
	return dist[100]
}

func main() {
	fmt.Println("Minimum turns:", minTurns())
}
