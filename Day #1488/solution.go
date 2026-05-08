// Day 1488: Topological sort of courses via Kahn's algorithm (BFS on in-degrees).
// Returns a valid ordering, or nil (=> null) if a cycle exists. Time O(V+E), Space O(V+E).
package main

import (
	"container/heap"
	"fmt"
	"strings"
)

type strHeap []string

func (h strHeap) Len() int            { return len(h) }
func (h strHeap) Less(i, j int) bool  { return h[i] < h[j] }
func (h strHeap) Swap(i, j int)       { h[i], h[j] = h[j], h[i] }
func (h *strHeap) Push(x interface{}) { *h = append(*h, x.(string)) }
func (h *strHeap) Pop() interface{} {
	old := *h
	n := len(old)
	v := old[n-1]
	*h = old[:n-1]
	return v
}

// prereqs[course] = list of its prerequisites. Returns (nil,false) on cycle.
func topoSort(prereqs map[string][]string) ([]string, bool) {
	indeg := map[string]int{}
	adj := map[string][]string{}
	for course, ps := range prereqs {
		if _, ok := indeg[course]; !ok {
			indeg[course] = 0
		}
		for _, p := range ps {
			if _, ok := indeg[p]; !ok {
				indeg[p] = 0
			}
		}
	}
	for course, ps := range prereqs {
		for _, p := range ps {
			adj[p] = append(adj[p], course) // p before course
			indeg[course]++
		}
	}

	h := &strHeap{}
	for c, d := range indeg {
		if d == 0 {
			heap.Push(h, c)
		}
	}
	order := []string{}
	for h.Len() > 0 {
		c := heap.Pop(h).(string)
		order = append(order, c)
		for _, nxt := range adj[c] {
			indeg[nxt]--
			if indeg[nxt] == 0 {
				heap.Push(h, nxt)
			}
		}
	}
	if len(order) != len(indeg) {
		return nil, false
	}
	return order, true
}

func main() {
	prereqs := map[string][]string{
		"CSC300": {"CSC100", "CSC200"},
		"CSC200": {"CSC100"},
		"CSC100": {},
	}
	order, ok := topoSort(prereqs)
	if !ok {
		fmt.Println("null")
		return
	}
	parts := make([]string, len(order))
	for i, c := range order {
		parts[i] = "'" + c + "'"
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
