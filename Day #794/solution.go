// Max stack with O(1) push/pop/max using an auxiliary stack of running maxima.
// All operations O(1) time; O(n) space.
package main

import "fmt"

type MaxStack struct {
	data []int
	maxs []int
}

func (s *MaxStack) Push(val int) {
	s.data = append(s.data, val)
	if len(s.maxs) == 0 || val >= s.maxs[len(s.maxs)-1] {
		s.maxs = append(s.maxs, val)
	} else {
		s.maxs = append(s.maxs, s.maxs[len(s.maxs)-1])
	}
}

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

func main() {
	s := &MaxStack{}
	for _, v := range []int{1, 5, 3, 9, 2} {
		s.Push(v)
	}
	m, _ := s.Max()
	fmt.Println("max:", m)
	p, _ := s.Pop()
	fmt.Println("pop:", p)
	p, _ = s.Pop()
	fmt.Println("pop:", p)
	m, _ = s.Max()
	fmt.Println("max:", m)
}
