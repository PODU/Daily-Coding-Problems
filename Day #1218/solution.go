// Three stacks in one array via fixed equal regions, each with its own top. O(1) push/pop.
package main

import "fmt"

type ThreeStacks struct {
	cap int
	arr []int
	top [3]int
}

func NewThreeStacks(perStack int) *ThreeStacks {
	return &ThreeStacks{cap: perStack, arr: make([]int, 3*perStack)}
}

func (t *ThreeStacks) Push(item, sn int) {
	if t.top[sn] >= t.cap {
		panic("stack full")
	}
	t.arr[sn*t.cap+t.top[sn]] = item
	t.top[sn]++
}

func (t *ThreeStacks) Pop(sn int) int {
	if t.top[sn] <= 0 {
		panic("stack empty")
	}
	t.top[sn]--
	return t.arr[sn*t.cap+t.top[sn]]
}

func main() {
	s := NewThreeStacks(3)
	s.Push(1, 0)
	s.Push(2, 0)
	s.Push(10, 1)
	s.Push(20, 1)
	s.Push(100, 2)
	fmt.Println("stack0 pop:", s.Pop(0))
	fmt.Println("stack1 pop:", s.Pop(1))
	fmt.Println("stack2 pop:", s.Pop(2))
	fmt.Println("stack0 pop:", s.Pop(0))
}
