// Day 438: Stack via a max-heap. Each push tags the item with an increasing
// counter; the heap's max counter is the most-recent item. push/pop O(log n).
package main

import (
	"container/heap"
	"fmt"
)

type entry struct {
	counter int64
	value   int
}

type maxHeap []entry

func (h maxHeap) Len() int            { return len(h) }
func (h maxHeap) Less(i, j int) bool  { return h[i].counter > h[j].counter }
func (h maxHeap) Swap(i, j int)       { h[i], h[j] = h[j], h[i] }
func (h *maxHeap) Push(x interface{}) { *h = append(*h, x.(entry)) }
func (h *maxHeap) Pop() interface{} {
	old := *h
	n := len(old)
	e := old[n-1]
	*h = old[:n-1]
	return e
}

type StackFromHeap struct {
	h       maxHeap
	counter int64
}

func (s *StackFromHeap) push(item int) {
	heap.Push(&s.h, entry{s.counter, item})
	s.counter++
}

func (s *StackFromHeap) pop() int {
	if s.h.Len() == 0 {
		panic("stack is empty")
	}
	return heap.Pop(&s.h).(entry).value
}

func main() {
	s := &StackFromHeap{}
	s.push(1)
	s.push(2)
	s.push(3)
	fmt.Println(s.pop()) // 3
	fmt.Println(s.pop()) // 2
	s.push(4)
	fmt.Println(s.pop()) // 4
	fmt.Println(s.pop()) // 1
}
