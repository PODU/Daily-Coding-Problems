// Topological sort of courses (Kahn's algorithm with cycle detection).
// Lexicographic tie-break via min-heap for deterministic order.
// Time: O((V+E) log V), Space: O(V+E).
package main

import (
	"container/heap"
	"fmt"
	"strings"
)

type StrHeap []string

func (h StrHeap) Len() int            { return len(h) }
func (h StrHeap) Less(i, j int) bool  { return h[i] < h[j] }
func (h StrHeap) Swap(i, j int)       { h[i], h[j] = h[j], h[i] }
func (h *StrHeap) Push(x interface{}) { *h = append(*h, x.(string)) }
func (h *StrHeap) Pop() interface{} {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[:n-1]
	return x
}

func courseOrder(prereqs map[string][]string) []string {
	adj := map[string][]string{}
	indeg := map[string]int{}
	for c := range prereqs {
		indeg[c] = 0
	}
	for course, pres := range prereqs {
		for _, p := range pres {
			adj[p] = append(adj[p], course)
			indeg[course]++
			if _, ok := indeg[p]; !ok {
				indeg[p] = 0
			}
		}
	}
	h := &StrHeap{}
	for c, d := range indeg {
		if d == 0 {
			*h = append(*h, c)
		}
	}
	heap.Init(h)
	order := []string{}
	for h.Len() > 0 {
		c := heap.Pop(h).(string)
		order = append(order, c)
		for _, nx := range adj[c] {
			indeg[nx]--
			if indeg[nx] == 0 {
				heap.Push(h, nx)
			}
		}
	}
	if len(order) != len(indeg) {
		return nil
	}
	return order
}

func main() {
	prereqs := map[string][]string{
		"CSC300": {"CSC100", "CSC200"},
		"CSC200": {"CSC100"},
		"CSC100": {},
	}
	order := courseOrder(prereqs)
	if order == nil {
		fmt.Println("null")
		return
	}
	fmt.Println("['" + strings.Join(order, "', '") + "']") // ['CSC100', 'CSC200', 'CSC300']
}
