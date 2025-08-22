// K nearest points: max-heap of size k keyed by squared distance. Time O(n log k), Space O(k).
package main

import (
	"container/heap"
	"fmt"
	"sort"
)

type item struct {
	dist int
	idx  int
}

type maxHeap []item

func (h maxHeap) Len() int            { return len(h) }
func (h maxHeap) Less(i, j int) bool  { return h[i].dist > h[j].dist }
func (h maxHeap) Swap(i, j int)       { h[i], h[j] = h[j], h[i] }
func (h *maxHeap) Push(x interface{}) { *h = append(*h, x.(item)) }
func (h *maxHeap) Pop() interface{} {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[:n-1]
	return x
}

func kNearest(pts [][2]int, c [2]int, k int) [][2]int {
	h := &maxHeap{}
	heap.Init(h)
	for i, p := range pts {
		dx, dy := p[0]-c[0], p[1]-c[1]
		heap.Push(h, item{dx*dx + dy*dy, i})
		if h.Len() > k {
			heap.Pop(h)
		}
	}
	idx := []int{}
	for h.Len() > 0 {
		idx = append(idx, heap.Pop(h).(item).idx)
	}
	sort.Ints(idx) // keep original order for stable output
	res := [][2]int{}
	for _, i := range idx {
		res = append(res, pts[i])
	}
	return res
}

func main() {
	pts := [][2]int{{0, 0}, {5, 4}, {3, 1}}
	c := [2]int{1, 2}
	k := 2
	res := kNearest(pts, c, k)
	out := "["
	for i, p := range res {
		out += fmt.Sprintf("(%d, %d)", p[0], p[1])
		if i+1 < len(res) {
			out += ", "
		}
	}
	out += "]"
	fmt.Println(out)
}
