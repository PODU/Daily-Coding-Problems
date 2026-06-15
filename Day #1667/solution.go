// Day 1667: Course ordering via topological sort (Kahn's algorithm).
// Time O(V+E + V log V), Space O(V+E). Returns nil if a cycle exists.
package main

import (
	"container/heap"
	"fmt"
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
	indeg := map[string]int{}
	adj := map[string][]string{}
	for course, deps := range prereqs {
		if _, ok := indeg[course]; !ok {
			indeg[course] = 0
		}
		for _, d := range deps {
			if _, ok := indeg[d]; !ok {
				indeg[d] = 0
			}
		}
	}
	for course, deps := range prereqs {
		for _, d := range deps {
			adj[d] = append(adj[d], course)
			indeg[course]++
		}
	}
	h := &StrHeap{}
	for c, d := range indeg {
		if d == 0 {
			heap.Push(h, c)
		}
	}
	var order []string
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
		return nil
	}
	return order
}

func main() {
	g := map[string][]string{
		"CSC300": {"CSC100", "CSC200"},
		"CSC200": {"CSC100"},
		"CSC100": {},
	}
	fmt.Println(courseOrder(g)) // [CSC100 CSC200 CSC300]
}
