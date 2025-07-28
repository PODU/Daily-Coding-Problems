// Max Stack: main stack + auxiliary stack holding running max. All ops O(1).
// Time O(1) per op; Space O(n).
package main

import "fmt"

type MaxStack struct {
	data  []int
	maxes []int
}

func (s *MaxStack) Push(v int) {
	s.data = append(s.data, v)
	if len(s.maxes) == 0 || v > s.maxes[len(s.maxes)-1] {
		s.maxes = append(s.maxes, v)
	} else {
		s.maxes = append(s.maxes, s.maxes[len(s.maxes)-1])
	}
}

// Pop returns (top, ok); ok is false when empty.
func (s *MaxStack) Pop() (int, bool) {
	if len(s.data) == 0 {
		return 0, false
	}
	v := s.data[len(s.data)-1]
	s.data = s.data[:len(s.data)-1]
	s.maxes = s.maxes[:len(s.maxes)-1]
	return v, true
}

// Max returns (max, ok); ok is false when empty.
func (s *MaxStack) Max() (int, bool) {
	if len(s.maxes) == 0 {
		return 0, false
	}
	return s.maxes[len(s.maxes)-1], true
}

func main() {
	s := &MaxStack{}
	for _, v := range []int{3, 1, 5, 4} {
		s.Push(v)
		m, _ := s.Max()
		fmt.Printf("push %d -> max=%d\n", v, m)
	}
	p1, _ := s.Pop()
	m1, _ := s.Max()
	fmt.Printf("pop -> %d, max=%d\n", p1, m1)
	p2, _ := s.Pop()
	m2, _ := s.Max()
	fmt.Printf("pop -> %d, max=%d\n", p2, m2)
}
