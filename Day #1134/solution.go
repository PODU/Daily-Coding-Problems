// Running median with two heaps (max-heap low, min-heap high). O(log n) per insert.
package main

import (
	"container/heap"
	"fmt"
	"strconv"
)

type MaxHeap []int

func (h MaxHeap) Len() int            { return len(h) }
func (h MaxHeap) Less(i, j int) bool  { return h[i] > h[j] }
func (h MaxHeap) Swap(i, j int)       { h[i], h[j] = h[j], h[i] }
func (h *MaxHeap) Push(x interface{}) { *h = append(*h, x.(int)) }
func (h *MaxHeap) Pop() interface{} {
	old := *h
	n := len(old)
	v := old[n-1]
	*h = old[:n-1]
	return v
}

type MinHeap []int

func (h MinHeap) Len() int            { return len(h) }
func (h MinHeap) Less(i, j int) bool  { return h[i] < h[j] }
func (h MinHeap) Swap(i, j int)       { h[i], h[j] = h[j], h[i] }
func (h *MinHeap) Push(x interface{}) { *h = append(*h, x.(int)) }
func (h *MinHeap) Pop() interface{} {
	old := *h
	n := len(old)
	v := old[n-1]
	*h = old[:n-1]
	return v
}

func fmtMedian(sum int, even bool) string {
	if !even {
		return strconv.Itoa(sum)
	}
	if sum%2 == 0 {
		return strconv.Itoa(sum / 2)
	}
	return strconv.FormatFloat(float64(sum)/2.0, 'g', -1, 64)
}

func main() {
	nums := []int{2, 1, 5, 7, 2, 0, 5}
	low := &MaxHeap{}
	high := &MinHeap{}
	heap.Init(low)
	heap.Init(high)

	for _, x := range nums {
		if low.Len() == 0 || x <= (*low)[0] {
			heap.Push(low, x)
		} else {
			heap.Push(high, x)
		}
		if low.Len() > high.Len()+1 {
			heap.Push(high, heap.Pop(low))
		} else if high.Len() > low.Len() {
			heap.Push(low, heap.Pop(high))
		}

		if low.Len() == high.Len() {
			fmt.Println(fmtMedian((*low)[0]+(*high)[0], true))
		} else {
			fmt.Println(fmtMedian((*low)[0], false))
		}
	}
}
