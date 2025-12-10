// Day 727: Running median over a stream.
// Approach: Max-heap for lower half, min-heap for upper half, kept balanced.
// Time: O(log n) per element, Space: O(n).
package main

import (
	"container/heap"
	"fmt"
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
	n := len(h.data)
	v := h.data[n-1]
	h.data = h.data[:n-1]
	return v
}
func (h IntHeap) Peek() int { return h.data[0] }

func printMedian(m float64) {
	if m == float64(int64(m)) {
		fmt.Println(int64(m))
	} else {
		fmt.Printf("%.1f\n", m)
	}
}

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
		if lo.Len() == hi.Len() {
			printMedian(float64(lo.Peek()+hi.Peek()) / 2.0)
		} else {
			printMedian(float64(lo.Peek()))
		}
	}
}
