// Sort a k-sorted array with a min-heap of size k+1. Time O(N log k), Space O(k).
package main

import (
	"container/heap"
	"fmt"
)

type IntHeap []int

func (h IntHeap) Len() int            { return len(h) }
func (h IntHeap) Less(i, j int) bool  { return h[i] < h[j] }
func (h IntHeap) Swap(i, j int)       { h[i], h[j] = h[j], h[i] }
func (h *IntHeap) Push(x interface{}) { *h = append(*h, x.(int)) }
func (h *IntHeap) Pop() interface{} {
	old := *h
	n := len(old)
	v := old[n-1]
	*h = old[:n-1]
	return v
}

func sortKSorted(a []int, k int) []int {
	h := &IntHeap{}
	heap.Init(h)
	res := make([]int, 0, len(a))
	for _, x := range a {
		heap.Push(h, x)
		if h.Len() > k {
			res = append(res, heap.Pop(h).(int))
		}
	}
	for h.Len() > 0 {
		res = append(res, heap.Pop(h).(int))
	}
	return res
}

func main() {
	a := []int{6, 5, 3, 2, 8, 10, 9}
	fmt.Println(sortKSorted(a, 3))
}
