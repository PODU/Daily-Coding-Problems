// Max stack with O(1) push/pop/max.
// Auxiliary stack stores running maxima alongside main stack. All ops O(1); space O(N).
package main

import (
	"fmt"
	"strconv"
)

type MaxStack struct {
	data []int
	maxs []int
}

func (s *MaxStack) Push(val int) {
	s.data = append(s.data, val)
	if len(s.maxs) == 0 || val > s.maxs[len(s.maxs)-1] {
		s.maxs = append(s.maxs, val)
	} else {
		s.maxs = append(s.maxs, s.maxs[len(s.maxs)-1])
	}
}

// Pop returns (top, true) or (0, false) if empty.
func (s *MaxStack) Pop() (int, bool) {
	if len(s.data) == 0 {
		return 0, false
	}
	v := s.data[len(s.data)-1]
	s.data = s.data[:len(s.data)-1]
	s.maxs = s.maxs[:len(s.maxs)-1]
	return v, true
}

func (s *MaxStack) Max() (int, bool) {
	if len(s.maxs) == 0 {
		return 0, false
	}
	return s.maxs[len(s.maxs)-1], true
}

func show(v int, ok bool) string {
	if !ok {
		return "null"
	}
	return strconv.Itoa(v)
}

func main() {
	s := &MaxStack{}
	s.Push(3)
	s.Push(1)
	s.Push(5)
	s.Push(2)
	fmt.Println("max:", show(s.Max()))
	fmt.Println("pop:", show(s.Pop()))
	fmt.Println("pop:", show(s.Pop()))
	fmt.Println("max:", show(s.Max()))
}
