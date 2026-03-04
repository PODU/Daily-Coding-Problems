// Day 1150: Skyline - sweep line over building edges with a max-heap (lazy deletion).
// Track current max height; emit point when it changes. Time O(n log n), Space O(n).
package main

import (
	"container/heap"
	"fmt"
	"sort"
	"strings"
)

type MaxHeap []int

func (h MaxHeap) Len() int            { return len(h) }
func (h MaxHeap) Less(i, j int) bool  { return h[i] > h[j] }
func (h MaxHeap) Swap(i, j int)       { h[i], h[j] = h[j], h[i] }
func (h *MaxHeap) Push(x interface{}) { *h = append(*h, x.(int)) }
func (h *MaxHeap) Pop() interface{} {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[:n-1]
	return x
}

func skyline(buildings [][3]int) [][2]int {
	type ev struct{ x, h int }
	var events []ev
	for _, b := range buildings {
		events = append(events, ev{b[0], -b[2]}, ev{b[1], b[2]})
	}
	sort.Slice(events, func(i, j int) bool {
		if events[i].x != events[j].x {
			return events[i].x < events[j].x
		}
		return events[i].h < events[j].h
	})
	h := &MaxHeap{0}
	removed := map[int]int{}
	prev := 0
	var res [][2]int
	for _, e := range events {
		if e.h < 0 {
			heap.Push(h, -e.h)
		} else {
			removed[e.h]++
		}
		for h.Len() > 0 && removed[(*h)[0]] > 0 {
			removed[(*h)[0]]--
			heap.Pop(h)
		}
		cur := (*h)[0]
		if cur != prev {
			res = append(res, [2]int{e.x, cur})
			prev = cur
		}
	}
	return res
}

func main() {
	bld := [][3]int{{0, 15, 3}, {4, 11, 5}, {19, 23, 4}}
	sk := skyline(bld)
	var parts []string
	for _, p := range sk {
		parts = append(parts, fmt.Sprintf("(%d, %d)", p[0], p[1]))
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
	// [(0, 3), (4, 5), (11, 3), (15, 0), (19, 4), (23, 0)]
}
