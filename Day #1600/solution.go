// Stack via a single MAX-heap keyed by a monotonic counter; newest key is largest so pops first (LIFO).
// push/pop O(log n), space O(n).
package main

import (
	"container/heap"
	"fmt"
	"strings"
)

type entry struct {
	key int64
	val int
}

// maxHeap orders by key descending (largest key at top).
type maxHeap []entry

func (h maxHeap) Len() int            { return len(h) }
func (h maxHeap) Less(i, j int) bool  { return h[i].key > h[j].key }
func (h maxHeap) Swap(i, j int)       { h[i], h[j] = h[j], h[i] }
func (h *maxHeap) Push(x interface{}) { *h = append(*h, x.(entry)) }
func (h *maxHeap) Pop() interface{} {
	old := *h
	n := len(old)
	e := old[n-1]
	*h = old[:n-1]
	return e
}

type StackViaHeap struct {
	h       maxHeap
	counter int64
}

func NewStack() *StackViaHeap {
	s := &StackViaHeap{}
	heap.Init(&s.h)
	return s
}

func (s *StackViaHeap) Push(x int) {
	heap.Push(&s.h, entry{key: s.counter, val: x})
	s.counter++
}

func (s *StackViaHeap) Pop() int {
	if s.h.Len() == 0 {
		panic("pop from empty stack")
	}
	return heap.Pop(&s.h).(entry).val
}

func main() {
	s := NewStack()
	s.Push(1)
	s.Push(2)
	s.Push(3)
	var out []int
	out = append(out, s.Pop()) // 3
	out = append(out, s.Pop()) // 2
	s.Push(4)
	out = append(out, s.Pop()) // 4
	out = append(out, s.Pop()) // 1

	parts := make([]string, len(out))
	for i, v := range out {
		parts[i] = fmt.Sprintf("%d", v)
	}
	fmt.Println(strings.Join(parts, " "))
}
