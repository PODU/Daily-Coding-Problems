// Minimum cost to connect all houses to plant = Minimum Spanning Tree weight.
// Prim's algorithm with a min-heap. Time O(E log V), Space O(V + E).
package main

import (
	"container/heap"
	"fmt"
)

type item struct {
	w    int
	node string
}
type pq []item

func (p pq) Len() int            { return len(p) }
func (p pq) Less(i, j int) bool  { return p[i].w < p[j].w }
func (p pq) Swap(i, j int)       { p[i], p[j] = p[j], p[i] }
func (p *pq) Push(x interface{}) { *p = append(*p, x.(item)) }
func (p *pq) Pop() interface{} {
	old := *p
	n := len(old)
	it := old[n-1]
	*p = old[:n-1]
	return it
}

func minCost(g map[string]map[string]int) int {
	if len(g) == 0 {
		return 0
	}
	var start string
	for k := range g {
		start = k
		break
	}
	visited := map[string]bool{}
	h := &pq{{0, start}}
	total := 0
	for h.Len() > 0 {
		cur := heap.Pop(h).(item)
		if visited[cur.node] {
			continue
		}
		visited[cur.node] = true
		total += cur.w
		for v, c := range g[cur.node] {
			if !visited[v] {
				heap.Push(h, item{c, v})
			}
		}
	}
	return total
}

func addEdge(g map[string]map[string]int, a, b string, c int) {
	if g[a] == nil {
		g[a] = map[string]int{}
	}
	if g[b] == nil {
		g[b] = map[string]int{}
	}
	g[a][b] = c
	g[b][a] = c
}

func main() {
	g := map[string]map[string]int{}
	addEdge(g, "plant", "A", 1)
	addEdge(g, "plant", "B", 5)
	addEdge(g, "plant", "C", 20)
	addEdge(g, "A", "C", 15)
	addEdge(g, "B", "C", 10)
	fmt.Println(minCost(g)) // 16
}
