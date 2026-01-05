// Day 858: Running median of a stream.
// Approach: max-heap (lower half) + min-heap (upper half), balanced sizes.
// Time: O(n log n) total, Space: O(n).
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
func (h IntHeap) Swap(i, j int)      { h.data[i], h.data[j] = h.data[j], h.data[i] }
func (h *IntHeap) Push(x interface{}) { h.data = append(h.data, x.(int)) }
func (h *IntHeap) Pop() interface{} {
	old := h.data
	n := len(old)
	v := old[n-1]
	h.data = old[:n-1]
	return v
}
func (h IntHeap) peek() int { return h.data[0] }

func main() {
	stream := []int{2, 1, 5, 7, 2, 0, 5}
	lo := &IntHeap{max: true}
	hi := &IntHeap{max: false}

	for _, x := range stream {
		if lo.Len() == 0 || x <= lo.peek() {
			heap.Push(lo, x)
		} else {
			heap.Push(hi, x)
		}
		if lo.Len() > hi.Len()+1 {
			heap.Push(hi, heap.Pop(lo))
		} else if hi.Len() > lo.Len() {
			heap.Push(lo, heap.Pop(hi))
		}
		var med float64
		if lo.Len() > hi.Len() {
			med = float64(lo.peek())
		} else {
			med = float64(lo.peek()+hi.peek()) / 2.0
		}
		if med == math.Floor(med) {
			fmt.Println(int64(med))
		} else {
			fmt.Println(med)
		}
	}
}
