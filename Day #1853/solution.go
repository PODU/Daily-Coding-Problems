// Day 1853: Stack with O(1) push/pop/max.
// Keep an auxiliary stack of running maxima alongside the data stack. All ops O(1) time, O(n) space.
package main

import (
	"errors"
	"fmt"
)

type MaxStack struct {
	data []int
	maxs []int
}

func (s *MaxStack) Push(v int) {
	s.data = append(s.data, v)
	if len(s.maxs) == 0 || v > s.maxs[len(s.maxs)-1] {
		s.maxs = append(s.maxs, v)
	} else {
		s.maxs = append(s.maxs, s.maxs[len(s.maxs)-1])
	}
}

func (s *MaxStack) Pop() (int, error) {
	if len(s.data) == 0 {
		return 0, errors.New("empty stack")
	}
	v := s.data[len(s.data)-1]
	s.data = s.data[:len(s.data)-1]
	s.maxs = s.maxs[:len(s.maxs)-1]
	return v, nil
}

func (s *MaxStack) Max() (int, error) {
	if len(s.maxs) == 0 {
		return 0, errors.New("empty stack")
	}
	return s.maxs[len(s.maxs)-1], nil
}

func main() {
	s := &MaxStack{}
	s.Push(1); s.Push(5); s.Push(3)
	m, _ := s.Max(); fmt.Println(m) // 5
	v, _ := s.Pop(); fmt.Println(v) // 3
	m, _ = s.Max(); fmt.Println(m)  // 5
	v, _ = s.Pop(); fmt.Println(v)  // 5
	m, _ = s.Max(); fmt.Println(m)  // 1
}
