// Running median with two heaps (max-heap for lower half, min-heap for upper).
// O(log n) per element.
package main

import (
	"container/heap"
	"fmt"
	"math"
)

type IntHeap struct {
	data []int
	max  bool
}

func (h IntHeap) Len() int { return len(h.data) }
func (h IntHeap) Less(i, j int) bool {
	if h.max {
		return h.data[i] > h.data[j]
	}
	return h.data[i] < h.data[j]
}
func (h IntHeap) Swap(i, j int)       { h.data[i], h.data[j] = h.data[j], h.data[i] }
func (h *IntHeap) Push(x interface{}) { h.data = append(h.data, x.(int)) }
func (h *IntHeap) Pop() interface{} {
	old := h.data
	n := len(old)
	x := old[n-1]
	h.data = old[:n-1]
	return x
}
func (h IntHeap) Peek() int { return h.data[0] }

func main() {
	stream := []int{2, 1, 5, 7, 2, 0, 5}
	lo := &IntHeap{max: true}
	hi := &IntHeap{max: false}
	heap.Init(lo)
	heap.Init(hi)

	for _, x := range stream {
		if lo.Len() == 0 || x <= lo.Peek() {
			heap.Push(lo, x)
		} else {
			heap.Push(hi, x)
		}
		if lo.Len() > hi.Len()+1 {
			heap.Push(hi, heap.Pop(lo))
		} else if hi.Len() > lo.Len() {
			heap.Push(lo, heap.Pop(hi))
		}

		var m float64
		if lo.Len() > hi.Len() {
			m = float64(lo.Peek())
		} else {
			m = float64(lo.Peek()+hi.Peek()) / 2.0
		}
		if math.Abs(m-math.Round(m)) < 1e-9 {
			fmt.Println(int64(math.Round(m)))
		} else {
			fmt.Println(m)
		}
	}
}
