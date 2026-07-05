// Uphill-then-downhill shortest cyclic route from home (node 0): Dijkstra over
// states (node, phase). UP edges require rising elevation, DOWN edges require
// falling; a free phase switch (the peak) is allowed at non-home nodes.
// Time: O(E log V), Space: O(V+E).
package main

import (
	"container/heap"
	"fmt"
)

type Item struct {
	dist, state int
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

func main() {
	elev := map[int]int{0: 5, 1: 25, 2: 15, 3: 20, 4: 10}
	type edge struct{ u, v, w int }
	paths := []edge{
		{0, 1, 10}, {0, 2, 8}, {0, 3, 15}, {1, 3, 12},
		{2, 4, 10}, {3, 4, 5}, {3, 0, 17}, {4, 0, 10},
	}
	n := len(elev)
	adj := make([][][2]int, n)
	for _, e := range paths {
		adj[e.u] = append(adj[e.u], [2]int{e.v, e.w})
	}

	const INF = 1 << 60
	dist := make([]int, 2*n)
	for i := range dist {
		dist[i] = INF
	}
	home := 0
	dist[home*2] = 0
	pq := &PQ{{0, home * 2}}
	for pq.Len() > 0 {
		it := heap.Pop(pq).(Item)
		d, s := it.dist, it.state
		if d > dist[s] {
			continue
		}
		node, phase := s/2, s%2
		if phase == 0 && node != home {
			ns := node*2 + 1
			if d < dist[ns] {
				dist[ns] = d
				heap.Push(pq, Item{d, ns})
			}
		}
		for _, e := range adj[node] {
			v, w := e[0], e[1]
			if phase == 0 && elev[v] > elev[node] {
				ns := v * 2
				if d+w < dist[ns] {
					dist[ns] = d + w
					heap.Push(pq, Item{d + w, ns})
				}
			}
			if phase == 1 && elev[v] < elev[node] {
				ns := v*2 + 1
				if d+w < dist[ns] {
					dist[ns] = d + w
					heap.Push(pq, Item{d + w, ns})
				}
			}
		}
	}
	fmt.Println(dist[home*2+1])
}
