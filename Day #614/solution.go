// Day 614: Time for a message from node 0 to reach all nodes = max shortest-path distance.
// Approach: Dijkstra from node 0, return the largest distance. Time O(E log V), Space O(V+E).
package main

import (
	"container/heap"
	"fmt"
	"math"
)

type Item struct {
	dist int64
	node int
}
type PQ []Item

func (p PQ) Len() int            { return len(p) }
func (p PQ) Less(i, j int) bool  { return p[i].dist < p[j].dist }
func (p PQ) Swap(i, j int)       { p[i], p[j] = p[j], p[i] }
func (p *PQ) Push(x interface{}) { *p = append(*p, x.(Item)) }
func (p *PQ) Pop() interface{} {
	old := *p
	n := len(old)
	it := old[n-1]
	*p = old[:n-1]
	return it
}

func broadcastTime(n int, edges [][3]int) int64 {
	adj := make([][][2]int, n+1)
	for _, e := range edges {
		adj[e[0]] = append(adj[e[0]], [2]int{e[1], e[2]})
	}
	dist := make([]int64, n+1)
	for i := range dist {
		dist[i] = math.MaxInt64
	}
	dist[0] = 0
	pq := &PQ{{0, 0}}
	for pq.Len() > 0 {
		cur := heap.Pop(pq).(Item)
		if cur.dist > dist[cur.node] {
			continue
		}
		for _, e := range adj[cur.node] {
			v, w := e[0], int64(e[1])
			if cur.dist+w < dist[v] {
				dist[v] = cur.dist + w
				heap.Push(pq, Item{dist[v], v})
			}
		}
	}
	var ans int64 = 0
	for _, d := range dist {
		if d > ans {
			ans = d
		}
	}
	return ans
}

func main() {
	N := 5
	edges := [][3]int{
		{0, 1, 5}, {0, 2, 3}, {0, 5, 4}, {1, 3, 8},
		{2, 3, 1}, {3, 5, 10}, {3, 4, 5},
	}
	fmt.Println(broadcastTime(N, edges)) // 9
}
