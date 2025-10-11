// Day 407: Minimum Spanning Tree of water pipes (Prim's algorithm).
// Approach: Prim with a min-heap over an undirected weighted graph.
// Time: O(E log V), Space: O(V + E). Example MST total cost = 16.
package main

import (
	"container/heap"
	"fmt"
)

type item struct {
	cost int
	node string
}
type minHeap []item

func (h minHeap) Len() int            { return len(h) }
func (h minHeap) Less(i, j int) bool  { return h[i].cost < h[j].cost }
func (h minHeap) Swap(i, j int)       { h[i], h[j] = h[j], h[i] }
func (h *minHeap) Push(x interface{}) { *h = append(*h, x.(item)) }
func (h *minHeap) Pop() interface{} {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[:n-1]
	return x
}

func minimumCost(pipes map[string]map[string]int) int {
	adj := map[string][]item{}
	for node := range pipes {
		if _, ok := adj[node]; !ok {
			adj[node] = nil
		}
	}
	for u, nbrs := range pipes {
		for v, w := range nbrs {
			adj[u] = append(adj[u], item{w, v})
			adj[v] = append(adj[v], item{w, u})
		}
	}
	if len(adj) == 0 {
		return 0
	}
	var start string
	for k := range adj {
		start = k
		break
	}
	visited := map[string]bool{}
	h := &minHeap{{0, start}}
	total := 0
	for h.Len() > 0 {
		cur := heap.Pop(h).(item)
		if visited[cur.node] {
			continue
		}
		visited[cur.node] = true
		total += cur.cost
		for _, e := range adj[cur.node] {
			if !visited[e.node] {
				heap.Push(h, e)
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
	fmt.Println(minimumCost(pipes)) // 16
}
