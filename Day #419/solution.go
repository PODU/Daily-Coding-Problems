// Day 419: Min steps to reduce N to 1 (decrement, or jump to larger factor).
// BFS over values 1..N. Time: O(N*sqrt(N)), Space: O(N).
package main

import "fmt"

func minSteps(N int) int {
	if N <= 1 {
		return 0
	}
	dist := make([]int, N+1)
	for i := range dist {
		dist[i] = -1
	}
	dist[N] = 0
	q := []int{N}
	for head := 0; head < len(q); head++ {
		v := q[head]
		if v == 1 {
			return dist[1]
		}
		if v-1 >= 1 && dist[v-1] == -1 {
			dist[v-1] = dist[v] + 1
			q = append(q, v-1)
		}
		for a := 2; a*a <= v; a++ {
			if v%a == 0 {
				larger := v / a
				if dist[larger] == -1 {
					dist[larger] = dist[v] + 1
					q = append(q, larger)
				}
			}
		}
	}
	return dist[1]
}

func main() {
	N := 100
	fmt.Printf("%d  (route: 100 -> 10 -> 9 -> 3 -> 2 -> 1)\n", minSteps(N))
}
