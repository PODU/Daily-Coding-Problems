// Snakes & Ladders minimum turns: BFS over board squares (unweighted shortest path).
// Each square has up to 6 die-roll edges; snakes/ladders redirect the landing square.
// Time: O(100*6). Space: O(100).
package main

import "fmt"

func minTurns(jump map[int]int, size int) int {
	dist := make([]int, size+1)
	for i := range dist {
		dist[i] = -1
	}
	dist[1] = 0
	q := []int{1} // begin on square 1; jumps trigger only on rolled squares
	for len(q) > 0 {
		sq := q[0]
		q = q[1:]
		if sq == size {
			return dist[sq]
		}
		for d := 1; d <= 6; d++ {
			nxt := sq + d
			if nxt > size {
				continue
			}
			if j, ok := jump[nxt]; ok {
				nxt = j
			}
			if dist[nxt] == -1 {
				dist[nxt] = dist[sq] + 1
				q = append(q, nxt)
			}
		}
	}
	return dist[size]
}

func main() {
	jump := map[int]int{
		16: 6, 48: 26, 49: 11, 56: 53, 62: 19, 64: 60, 87: 24, 93: 73, 95: 75, 98: 78,
		1: 38, 4: 14, 9: 31, 21: 42, 28: 84, 36: 44, 51: 67, 71: 91, 80: 100,
	}
	fmt.Println(minTurns(jump, 100)) // 7
}
