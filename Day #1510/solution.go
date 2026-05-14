// Three stacks sharing ONE backing slice of nodes (value, prevIndex) + free list.
// Three head pointers index into the single shared list. O(1) push/pop, O(n) space.
package main

import (
	"fmt"
	"strings"
)

type ThreeStacks struct {
	val      []int
	prev     []int
	head     [3]int
	freeHead int
}

func NewThreeStacks() *ThreeStacks {
	return &ThreeStacks{head: [3]int{-1, -1, -1}, freeHead: -1}
}

func (t *ThreeStacks) alloc(v, p int) int {
	var idx int
	if t.freeHead != -1 {
		idx = t.freeHead
		t.freeHead = t.prev[idx]
		t.val[idx] = v
		t.prev[idx] = p
	} else {
		idx = len(t.val)
		t.val = append(t.val, v)
		t.prev = append(t.prev, p)
	}
	return idx
}

func (t *ThreeStacks) Push(item, s int) {
	t.head[s] = t.alloc(item, t.head[s])
}

func (t *ThreeStacks) Pop(s int) int {
	idx := t.head[s]
	v := t.val[idx]
	t.head[s] = t.prev[idx]
	t.prev[idx] = t.freeHead
	t.freeHead = idx
	return v
}

func main() {
	st := NewThreeStacks()
	st.Push(1, 0)
	st.Push(2, 0)
	st.Push(3, 1)
	st.Push(4, 2)
	st.Push(5, 2)
	res := []string{
		fmt.Sprint(st.Pop(0)),
		fmt.Sprint(st.Pop(2)),
		fmt.Sprint(st.Pop(1)),
		fmt.Sprint(st.Pop(0)),
	}
	fmt.Println(strings.Join(res, " "))
}
