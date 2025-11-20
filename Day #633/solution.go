// Day 633: Sort a k-sorted (nearly sorted) array.
// Approach: min-heap of size k+1; pop smallest as we slide the window.
// Time: O(N log k), Space: O(k).
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

func sortKSorted(arr []int, k int) []int {
	h := &IntHeap{}
	heap.Init(h)
	res := []int{}
	for _, x := range arr {
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
	arr := []int{2, 1, 4, 3, 5} // k = 1
	fmt.Println(sortKSorted(arr, 1))
}
