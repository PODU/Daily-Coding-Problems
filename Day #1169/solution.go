// K nearest points: max-heap of size k by squared Euclidean distance, then
// sort the k results by (dist, original index) for deterministic output.
// Time: O(n log k), Space: O(k).
package main

import (
	"container/heap"
	"fmt"
	"sort"
	"strings"
)

type Point struct{ x, y int64 }

type item struct {
	d   int64
	idx int
}

// max-heap by distance d
type maxHeap []item

func (h maxHeap) Len() int            { return len(h) }
func (h maxHeap) Less(i, j int) bool  { return h[i].d > h[j].d }
func (h maxHeap) Swap(i, j int)       { h[i], h[j] = h[j], h[i] }
func (h *maxHeap) Push(x interface{}) { *h = append(*h, x.(item)) }
func (h *maxHeap) Pop() interface{} {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[:n-1]
	return x
}

func main() {
	points := []Point{{0, 0}, {5, 4}, {3, 1}}
	cx, cy := int64(1), int64(2)
	k := 2

	dist2 := func(p Point) int64 {
		dx, dy := p.x-cx, p.y-cy
		return dx*dx + dy*dy
	}

	h := &maxHeap{}
	heap.Init(h)
	for i, p := range points {
		heap.Push(h, item{dist2(p), i})
		if h.Len() > k {
			heap.Pop(h)
		}
	}

	idx := make([]int, 0, h.Len())
	for _, it := range *h {
		idx = append(idx, it.idx)
	}
	sort.Slice(idx, func(a, b int) bool {
		da, db := dist2(points[idx[a]]), dist2(points[idx[b]])
		if da != db {
			return da < db
		}
		return idx[a] < idx[b]
	})

	parts := make([]string, 0, len(idx))
	for _, i := range idx {
		parts = append(parts, fmt.Sprintf("(%d, %d)", points[i].x, points[i].y))
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
