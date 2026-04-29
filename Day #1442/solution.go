// Day 1442: Implement a stack using only a max-heap.
// Approach: tag each item with an increasing counter as its key; the heap's
// max key is always the most recently pushed item. push/pop O(log n).
package main

import (
	"container/heap"
	"fmt"
)

type entry struct {
	key  int64
	val  int
}

type maxHeap []entry

func (h maxHeap) Len() int            { return len(h) }
func (h maxHeap) Less(i, j int) bool  { return h[i].key > h[j].key } // max-heap
func (h maxHeap) Swap(i, j int)       { h[i], h[j] = h[j], h[i] }
func (h *maxHeap) Push(x interface{}) { *h = append(*h, x.(entry)) }
func (h *maxHeap) Pop() interface{} {
	old := *h
	n := len(old)
	e := old[n-1]
	*h = old[:n-1]
	return e
}

type Stack struct {
	h       maxHeap
	counter int64
}

func (s *Stack) Push(item int) {
	heap.Push(&s.h, entry{s.counter, item})
	s.counter++
}

func (s *Stack) Pop() int {
	if s.h.Len() == 0 {
		panic("pop from empty stack")
	}
	return heap.Pop(&s.h).(entry).val
}

func main() {
	s := &Stack{}
	s.Push(1)
	s.Push(2)
	s.Push(3)
	fmt.Println(s.Pop(), s.Pop(), s.Pop()) // 3 2 1
}
