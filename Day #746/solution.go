// Max stack: each entry stores the running max so far, giving O(1) push/pop/max.
// Time: O(1) per operation, Space: O(n).
package main

import "fmt"

type entry struct{ val, mx int }

type MaxStack struct{ st []entry }

func (s *MaxStack) Push(v int) {
	mx := v
	if len(s.st) > 0 && s.st[len(s.st)-1].mx > mx {
		mx = s.st[len(s.st)-1].mx
	}
	s.st = append(s.st, entry{v, mx})
}

func (s *MaxStack) Pop() (int, bool) {
	if len(s.st) == 0 {
		return 0, false
	}
	v := s.st[len(s.st)-1].val
	s.st = s.st[:len(s.st)-1]
	return v, true
}

func (s *MaxStack) Max() (int, bool) {
	if len(s.st) == 0 {
		return 0, false
	}
	return s.st[len(s.st)-1].mx, true
}

func main() {
	s := &MaxStack{}
	s.Push(1)
	s.Push(3)
	s.Push(2)
	mx, _ := s.Max()
	fmt.Println("max:", mx) // 3
	v, _ := s.Pop()
	fmt.Println("pop:", v) // 2
	mx, _ = s.Max()
	fmt.Println("max:", mx) // 3
	v, _ = s.Pop()
	fmt.Println("pop:", v) // 3
	mx, _ = s.Max()
	fmt.Println("max:", mx) // 1
}
