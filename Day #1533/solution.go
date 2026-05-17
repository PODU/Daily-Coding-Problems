// Minimum spanning tree (Prim's algorithm) over an undirected pipe graph.
// Returns total cost to connect every house to the plant.
// Time O(E log V), Space O(V + E).
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

func mstCost(pipes map[string]map[string]int, order []string) int {
	adj := map[string][]item{}
	for _, u := range order {
		if _, ok := adj[u]; !ok {
			adj[u] = nil
		}
		for v, w := range pipes[u] {
			adj[u] = append(adj[u], item{w, v})
			adj[v] = append(adj[v], item{w, u})
		}
	}
	visited := map[string]bool{}
	h := &pq{{0, order[0]}}
	heap.Init(h)
	total := 0
	for h.Len() > 0 {
		cur := heap.Pop(h).(item)
		if visited[cur.node] {
			continue
		}
		visited[cur.node] = true
		total += cur.w
		for _, nb := range adj[cur.node] {
			if !visited[nb.node] {
				heap.Push(h, nb)
			}
		}
	}
	return total
}

func main() {
	pipes := map[string]map[string]int{
		"plant": {"A": 1, "B": 5, "C": 20},
		"A":     {"C": 15},
		"B":     {"C": 10},
		"C":     {},
	}
	order := []string{"plant", "A", "B", "C"}
	fmt.Println(mstCost(pipes, order))
}
