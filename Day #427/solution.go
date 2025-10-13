// Day 427: Shortest uphill-then-downhill route from/to home (location 0).
// State Dijkstra: each node split into up/down phases; switch at the peak.
// Up edges need strictly higher elevation, down edges strictly lower. Time O((V+E)logV).
package main

import (
	"container/heap"
	"fmt"
	"strconv"
	"strings"
)

type Item struct {
	d int64
	s int
}
type PQ []Item

func (p PQ) Len() int            { return len(p) }
func (p PQ) Less(i, j int) bool  { return p[i].d < p[j].d }
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
	edges := [][3]int{{0, 1, 10}, {0, 2, 8}, {0, 3, 15}, {1, 3, 12},
		{2, 4, 10}, {3, 4, 5}, {3, 0, 17}, {4, 0, 10}}
	n := len(elev)
	home := 0
	adj := make([][][2]int, n)
	for _, e := range edges {
		adj[e[0]] = append(adj[e[0]], [2]int{e[1], e[2]})
	}
	S := n * 2 // state = node*2 + phase (0 up, 1 down)
	const INF = int64(1) << 62
	dist := make([]int64, S)
	prev := make([]int, S)
	for i := range dist {
		dist[i] = INF
		prev[i] = -1
	}
	dist[home*2] = 0
	pq := &PQ{{0, home * 2}}
	heap.Init(pq)
	for pq.Len() > 0 {
		it := heap.Pop(pq).(Item)
		d, s := it.d, it.s
		if d > dist[s] {
			continue
		}
		u, ph := s/2, s%2
		if ph == 0 && u != home {
			ns := u*2 + 1
			if d < dist[ns] {
				dist[ns] = d
				prev[ns] = s
				heap.Push(pq, Item{d, ns})
			}
		}
		for _, pr := range adj[u] {
			v, w := pr[0], pr[1]
			var ns int
			if ph == 0 && elev[v] > elev[u] {
				ns = v * 2
			} else if ph == 1 && elev[v] < elev[u] {
				ns = v*2 + 1
			} else {
				continue
			}
			if d+int64(w) < dist[ns] {
				dist[ns] = d + int64(w)
				prev[ns] = s
				heap.Push(pq, Item{d + int64(w), ns})
			}
		}
	}
	goal := home*2 + 1
	var nodes []int
	for cur := goal; cur != -1; cur = prev[cur] {
		nodes = append(nodes, cur/2)
	}
	for i, j := 0, len(nodes)-1; i < j; i, j = i+1, j-1 {
		nodes[i], nodes[j] = nodes[j], nodes[i]
	}
	var path []int
	for _, x := range nodes {
		if len(path) == 0 || path[len(path)-1] != x {
			path = append(path, x)
		}
	}
	strs := make([]string, len(path))
	for i, x := range path {
		strs[i] = strconv.Itoa(x)
	}
	fmt.Printf("%s, distance %d\n", strings.Join(strs, " -> "), dist[goal])
}
