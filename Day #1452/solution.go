// Day 1452: Sort a k-sorted array (each element <= k from its place) using a
// min-heap of size k+1. Time O(N log k), Space O(k).
package main

import (
	"container/heap"
	"fmt"
)

type minHeap []int

func (h minHeap) Len() int            { return len(h) }
func (h minHeap) Less(i, j int) bool  { return h[i] < h[j] }
func (h minHeap) Swap(i, j int)       { h[i], h[j] = h[j], h[i] }
func (h *minHeap) Push(x interface{}) { *h = append(*h, x.(int)) }
func (h *minHeap) Pop() interface{} {
	old := *h
	n := len(old)
	v := old[n-1]
	*h = old[:n-1]
	return v
}

func sortKSorted(a []int, k int) []int {
	h := &minHeap{}
	out := make([]int, 0, len(a))
	for _, x := range a {
		heap.Push(h, x)
		if h.Len() > k {
			out = append(out, heap.Pop(h).(int))
		}
	}
	for h.Len() > 0 {
		out = append(out, heap.Pop(h).(int))
	}
	return out
}

func main() {
	a := []int{2, 6, 3, 12, 56, 8}
	fmt.Println(sortKSorted(a, 3)) // [2 3 6 8 12 56]
}
