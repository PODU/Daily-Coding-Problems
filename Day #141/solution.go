// Three stacks in one list via interleaved indexing: logical pos p of stack s -> physical p*3+s.
// push/pop O(1) amortized. Space O(total elements). Single backing list.
package main

import "fmt"

type Stack struct {
	list  []int  // single backing list
	sizes [3]int // logical height of each stack
}

func (s *Stack) Push(item, stackNumber int) {
	phys := s.sizes[stackNumber]*3 + stackNumber
	for len(s.list) <= phys {
		s.list = append(s.list, 0)
	}
	s.list[phys] = item
	s.sizes[stackNumber]++
}

func (s *Stack) Pop(stackNumber int) int {
	if s.sizes[stackNumber] == 0 {
		panic("empty stack")
	}
	s.sizes[stackNumber]--
	phys := s.sizes[stackNumber]*3 + stackNumber
	return s.list[phys]
}

func main() {
	var s Stack
	s.Push(1, 0)
	s.Push(2, 0)
	s.Push(10, 1)
	s.Push(100, 2)
	s.Push(200, 2)
	fmt.Println(s.Pop(0), s.Pop(2), s.Pop(1), s.Pop(2), s.Pop(0))
	// 2 200 10 100 1
}
