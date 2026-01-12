// Nearest k points: max-heap of size k by squared distance. Time O(n log k), Space O(k).
package main

import (
	"container/heap"
	"fmt"
	"sort"
	"strings"
)

type item struct {
	d2, x, y int
}

type maxHeap []item

func (h maxHeap) Len() int            { return len(h) }
func (h maxHeap) Less(i, j int) bool  { return h[i].d2 > h[j].d2 } // max on top
func (h maxHeap) Swap(i, j int)       { h[i], h[j] = h[j], h[i] }
func (h *maxHeap) Push(x interface{}) { *h = append(*h, x.(item)) }
func (h *maxHeap) Pop() interface{} {
	old := *h
	n := len(old)
	it := old[n-1]
	*h = old[:n-1]
	return it
}

func main() {
	points := [][2]int{{0, 0}, {5, 4}, {3, 1}}
	central := [2]int{1, 2}
	k := 2

	h := &maxHeap{}
	heap.Init(h)
	for _, p := range points {
		dx, dy := p[0]-central[0], p[1]-central[1]
		heap.Push(h, item{dx*dx + dy*dy, p[0], p[1]})
		if h.Len() > k {
			heap.Pop(h)
		}
	}
	res := make([][2]int, 0, h.Len())
	for _, it := range *h {
		res = append(res, [2]int{it.x, it.y})
	}
	sort.Slice(res, func(i, j int) bool {
		if res[i][0] != res[j][0] {
			return res[i][0] < res[j][0]
		}
		return res[i][1] < res[j][1]
	})
	parts := make([]string, len(res))
	for i, p := range res {
		parts[i] = fmt.Sprintf("(%d, %d)", p[0], p[1])
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
