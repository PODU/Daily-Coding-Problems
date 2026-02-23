// Day 1117 - Three stacks backed by a single list
// Each entry stores (value, prev_index); per-stack heads + free list give O(1)
// push/pop sharing one list. Space O(n).
package main

import "fmt"

type entry struct {
	value, prev int
}

type Stack3 struct {
	list  []entry
	heads [3]int
	free  []int
}

func NewStack3() *Stack3 {
	return &Stack3{heads: [3]int{-1, -1, -1}}
}

func (s *Stack3) Push(item, sn int) {
	var idx int
	if len(s.free) > 0 {
		idx = s.free[len(s.free)-1]
		s.free = s.free[:len(s.free)-1]
		s.list[idx] = entry{item, s.heads[sn]}
	} else {
		idx = len(s.list)
		s.list = append(s.list, entry{item, s.heads[sn]})
	}
	s.heads[sn] = idx
}

func (s *Stack3) Pop(sn int) int {
	idx := s.heads[sn]
	if idx == -1 {
		panic("pop from empty stack")
	}
	e := s.list[idx]
	s.heads[sn] = e.prev
	s.free = append(s.free, idx)
	return e.value
}

func main() {
	s := NewStack3()
	s.Push(1, 0)
	s.Push(2, 0)
	s.Push(3, 1)
	s.Push(4, 2)
	s.Push(5, 2)
	fmt.Println("pop(0):", s.Pop(0)) // 2
	fmt.Println("pop(0):", s.Pop(0)) // 1
	fmt.Println("pop(2):", s.Pop(2)) // 5
	fmt.Println("pop(1):", s.Pop(1)) // 3
	fmt.Println("pop(2):", s.Pop(2)) // 4
}
