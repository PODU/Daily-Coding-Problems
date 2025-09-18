// Uphill-then-downhill closed route: Dijkstra on up-edges from 0, Dijkstra on reversed down-edges
// to 0; answer = min over peaks of distUp[m]+distDown[m]. Time O((V+E)logV), Space O(V+E).
package main

import (
	"container/heap"
	"fmt"
	"math"
)

type Item struct {
	dist, node int
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

type Edge struct{ to, w int }

func dijkstra(n int, adj [][]Edge, src int) []int {
	d := make([]int, n)
	for i := range d {
		d[i] = math.MaxInt64
	}
	d[src] = 0
	pq := &PQ{{0, src}}
	for pq.Len() > 0 {
		it := heap.Pop(pq).(Item)
		if it.dist > d[it.node] {
			continue
		}
		for _, e := range adj[it.node] {
			if d[it.node]+e.w < d[e.to] {
				d[e.to] = d[it.node] + e.w
				heap.Push(pq, Item{d[e.to], e.to})
			}
		}
	}
	return d
}

func main() {
	n := 5
	elev := map[int]int{0: 5, 1: 25, 2: 15, 3: 20, 4: 10}
	type P struct{ u, v, w int }
	paths := []P{{0, 1, 10}, {0, 2, 8}, {0, 3, 15}, {1, 3, 12}, {2, 4, 10}, {3, 4, 5}, {3, 0, 17}, {4, 0, 10}}
	up := make([][]Edge, n)
	downRev := make([][]Edge, n)
	for _, p := range paths {
		if elev[p.v] > elev[p.u] {
			up[p.u] = append(up[p.u], Edge{p.v, p.w})
		}
		if elev[p.v] < elev[p.u] {
			downRev[p.v] = append(downRev[p.v], Edge{p.u, p.w})
		}
	}
	distUp := dijkstra(n, up, 0)
	distDown := dijkstra(n, downRev, 0)
	best := math.MaxInt64
	for m := 1; m < n; m++ {
		if distUp[m] != math.MaxInt64 && distDown[m] != math.MaxInt64 && distUp[m]+distDown[m] < best {
			best = distUp[m] + distDown[m]
		}
	}
	fmt.Println(best)
}
