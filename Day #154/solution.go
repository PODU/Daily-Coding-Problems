// Day 154: Stack using only a max-heap. Tag each item with an increasing
// priority so the heap always pops the most recent. push/pop O(log n).
package main

import (
	"container/heap"
	"fmt"
)

type entry struct {
	prio  int
	value int
}

type maxHeap []entry

func (h maxHeap) Len() int            { return len(h) }
func (h maxHeap) Less(i, j int) bool  { return h[i].prio > h[j].prio }
func (h maxHeap) Swap(i, j int)       { h[i], h[j] = h[j], h[i] }
func (h *maxHeap) Push(x interface{}) { *h = append(*h, x.(entry)) }
func (h *maxHeap) Pop() interface{} {
	old := *h
	n := len(old)
	e := old[n-1]
	*h = old[:n-1]
	return e
}

type HeapStack struct {
	h       *maxHeap
	counter int
}

func NewHeapStack() *HeapStack {
	h := &maxHeap{}
	heap.Init(h)
	return &HeapStack{h: h}
}

func (s *HeapStack) Push(item int) {
	heap.Push(s.h, entry{prio: s.counter, value: item})
	s.counter++
}

func (s *HeapStack) Pop() int {
	if s.h.Len() == 0 {
		panic("pop from empty stack")
	}
	return heap.Pop(s.h).(entry).value
}

func main() {
	s := NewHeapStack()
	s.Push(1)
	s.Push(2)
	s.Push(3)
	fmt.Println(s.Pop()) // 3
	fmt.Println(s.Pop()) // 2
	s.Push(4)
	fmt.Println(s.Pop()) // 4
	fmt.Println(s.Pop()) // 1
}
