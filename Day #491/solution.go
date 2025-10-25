// Day 491: Running median of a stream.
// Two heaps: a max-heap keeps the lower half, a min-heap the upper half; rebalance so
// the lower half has equal size or one extra, so the median is its top.
// Time: O(log n) per element, Space: O(n).
package main

import (
	"container/heap"
	"fmt"
	"math"
	"strconv"
)

type MaxHeap []int

func (h MaxHeap) Len() int           { return len(h) }
func (h MaxHeap) Less(i, j int) bool { return h[i] > h[j] }
func (h MaxHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *MaxHeap) Push(x any)        { *h = append(*h, x.(int)) }
func (h *MaxHeap) Pop() any          { o := *h; n := len(o); x := o[n-1]; *h = o[:n-1]; return x }

type MinHeap []int

func (h MinHeap) Len() int           { return len(h) }
func (h MinHeap) Less(i, j int) bool { return h[i] < h[j] }
func (h MinHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *MinHeap) Push(x any)        { *h = append(*h, x.(int)) }
func (h *MinHeap) Pop() any          { o := *h; n := len(o); x := o[n-1]; *h = o[:n-1]; return x }

func main() {
	stream := []int{2, 1, 5, 7, 2, 0, 5}
	lo := &MaxHeap{} // lower half
	hi := &MinHeap{} // upper half
	heap.Init(lo)
	heap.Init(hi)
	for _, x := range stream {
		if lo.Len() == 0 || x <= (*lo)[0] {
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
		if lo.Len() == hi.Len() {
			m = (float64((*lo)[0]) + float64((*hi)[0])) / 2
		} else {
			m = float64((*lo)[0])
		}
		// 2, 1.5, 2, 3.5, 2, 2, 2
		if m == math.Floor(m) {
			fmt.Println(strconv.FormatInt(int64(m), 10))
		} else {
			fmt.Println(strconv.FormatFloat(m, 'g', -1, 64))
		}
	}
}
