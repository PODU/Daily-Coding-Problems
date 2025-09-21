// Day 306: Sort a k-sorted array with a min-heap of size k+1. O(N log k).
package main

import (
	"container/heap"
	"fmt"
	"strings"
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

func sortK(arr []int, k int) []int {
	h := &IntHeap{}
	heap.Init(h)
	res := []int{}
	i := 0
	for ; i < len(arr) && i <= k; i++ {
		heap.Push(h, arr[i])
	}
	for ; i < len(arr); i++ {
		res = append(res, heap.Pop(h).(int))
		heap.Push(h, arr[i])
	}
	for h.Len() > 0 {
		res = append(res, heap.Pop(h).(int))
	}
	return res
}

func main() {
	r := sortK([]int{6, 5, 3, 2, 8, 10, 9}, 3)
	var s []string
	for _, v := range r {
		s = append(s, fmt.Sprintf("%d", v))
	}
	fmt.Println(strings.Join(s, " ")) // 2 3 5 6 8 9 10
}
