// Stack via max-heap: tag each push with increasing priority; heap pops highest priority (most recent). O(log n)/op, O(n) space.
package main

import (
	"container/heap"
	"fmt"
)

type item struct {
	priority int
	value    int
}

type maxHeap []item

func (h maxHeap) Len() int            { return len(h) }
func (h maxHeap) Less(i, j int) bool  { return h[i].priority > h[j].priority }
func (h maxHeap) Swap(i, j int)       { h[i], h[j] = h[j], h[i] }
func (h *maxHeap) Push(x interface{}) { *h = append(*h, x.(item)) }
func (h *maxHeap) Pop() interface{} {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[:n-1]
	return x
}

type HeapStack struct {
	h       maxHeap
	counter int
}

func NewHeapStack() *HeapStack { return &HeapStack{} }
func (s *HeapStack) Push(v int) {
	heap.Push(&s.h, item{priority: s.counter, value: v})
	s.counter++
}
func (s *HeapStack) Pop() int {
	if s.h.Len() == 0 {
		panic("pop from empty stack")
	}
	return heap.Pop(&s.h).(item).value
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
