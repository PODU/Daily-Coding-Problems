// Shortest uphill-then-downhill cycle from home (node 0).
// Dijkstra on two DAG subgraphs (uphill, reversed downhill), O(E log V) time, O(V+E) space.
package main

import (
	"container/heap"
	"fmt"
	"math"
)

type edge struct{ to, w int }

type item struct {
	dist, node int
}
type pq []item

func (p pq) Len() int            { return len(p) }
func (p pq) Less(i, j int) bool  { return p[i].dist < p[j].dist }
func (p pq) Swap(i, j int)       { p[i], p[j] = p[j], p[i] }
func (p *pq) Push(x interface{}) { *p = append(*p, x.(item)) }
func (p *pq) Pop() interface{} {
	old := *p
	n := len(old)
	it := old[n-1]
	*p = old[:n-1]
	return it
}

func dijkstra(adj [][]edge, src, n int) []int {
	dist := make([]int, n)
	for i := range dist {
		dist[i] = math.MaxInt64
	}
	dist[src] = 0
	q := &pq{{0, src}}
	for q.Len() > 0 {
		cur := heap.Pop(q).(item)
		if cur.dist > dist[cur.node] {
			continue
		}
		for _, e := range adj[cur.node] {
			if cur.dist+e.w < dist[e.to] {
				dist[e.to] = cur.dist + e.w
				heap.Push(q, item{dist[e.to], e.to})
			}
		}
	}
	return dist
}

func main() {
	elevations := map[int]int{0: 5, 1: 25, 2: 15, 3: 20, 4: 10}
	type p3 struct{ u, v, w int }
	paths := []p3{{0, 1, 10}, {0, 2, 8}, {0, 3, 15}, {1, 3, 12},
		{2, 4, 10}, {3, 4, 5}, {3, 0, 17}, {4, 0, 10}}
	n := 0
	for k := range elevations {
		if k+1 > n {
			n = k + 1
		}
	}
	up := make([][]edge, n)
	downRev := make([][]edge, n)
	for _, e := range paths {
		if elevations[e.v] > elevations[e.u] {
			up[e.u] = append(up[e.u], edge{e.v, e.w})
		} else if elevations[e.v] < elevations[e.u] {
			downRev[e.v] = append(downRev[e.v], edge{e.u, e.w})
		}
	}
	upD := dijkstra(up, 0, n)
	dnD := dijkstra(downRev, 0, n)
	best := math.MaxInt64
	for p := 1; p < n; p++ {
		if upD[p] != math.MaxInt64 && dnD[p] != math.MaxInt64 && upD[p]+dnD[p] < best {
			best = upD[p] + dnD[p]
		}
	}
	fmt.Println(best)
}
