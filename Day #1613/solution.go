// Skyline via sweep-line + max-heap (with lazy deletion). Emit point when max height changes.
// Time: O(n log n), Space: O(n).
package main

import (
	"container/heap"
	"fmt"
	"sort"
	"strings"
)

type maxHeap []int

func (h maxHeap) Len() int            { return len(h) }
func (h maxHeap) Less(i, j int) bool  { return h[i] > h[j] }
func (h maxHeap) Swap(i, j int)       { h[i], h[j] = h[j], h[i] }
func (h *maxHeap) Push(x interface{}) { *h = append(*h, x.(int)) }
func (h *maxHeap) Pop() interface{} {
	old := *h
	n := len(old)
	v := old[n-1]
	*h = old[:n-1]
	return v
}

func getSkyline(buildings [][3]int) [][2]int {
	type ev struct{ x, h int } // h: start = -height, end = +height
	events := []ev{}
	for _, b := range buildings {
		events = append(events, ev{b[0], -b[2]})
		events = append(events, ev{b[1], b[2]})
	}
	sort.Slice(events, func(i, j int) bool {
		if events[i].x != events[j].x {
			return events[i].x < events[j].x
		}
		return events[i].h < events[j].h
	})

	live := &maxHeap{0}
	heap.Init(live)
	removed := map[int]int{}
	prev := 0
	res := [][2]int{}
	for _, e := range events {
		if e.h < 0 {
			heap.Push(live, -e.h)
		} else {
			removed[e.h]++
		}
		for live.Len() > 0 && removed[(*live)[0]] > 0 {
			removed[(*live)[0]]--
			heap.Pop(live)
		}
		cur := (*live)[0]
		if cur != prev {
			res = append(res, [2]int{e.x, cur})
			prev = cur
		}
	}
	return res
}

func main() {
	buildings := [][3]int{{0, 15, 3}, {4, 11, 5}, {19, 23, 4}}
	res := getSkyline(buildings)
	parts := make([]string, len(res))
	for i, p := range res {
		parts[i] = fmt.Sprintf("(%d, %d)", p[0], p[1])
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
