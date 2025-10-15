// Day 436: Three stacks backed by one slice using node-based singly-linked
// indices + a free list. push/pop are O(1) time, O(n) space overall.
package main

import "fmt"

type node struct{ val, prev int }

type ThreeStacks struct {
	data []node
	tops [3]int
	free []int
}

func NewThreeStacks() *ThreeStacks {
	return &ThreeStacks{tops: [3]int{-1, -1, -1}}
}

func (t *ThreeStacks) push(item, s int) {
	var idx int
	if n := len(t.free); n > 0 {
		idx = t.free[n-1]
		t.free = t.free[:n-1]
		t.data[idx] = node{item, t.tops[s]}
	} else {
		idx = len(t.data)
		t.data = append(t.data, node{item, t.tops[s]})
	}
	t.tops[s] = idx
}

func (t *ThreeStacks) pop(s int) int {
	idx := t.tops[s]
	if idx == -1 {
		panic(fmt.Sprintf("stack %d is empty", s))
	}
	nd := t.data[idx]
	t.tops[s] = nd.prev
	t.free = append(t.free, idx)
	return nd.val
}

func main() {
	st := NewThreeStacks()
	st.push(1, 0)
	st.push(2, 0)
	st.push(10, 1)
	st.push(100, 2)
	st.push(200, 2)
	fmt.Println(st.pop(0)) // 2
	fmt.Println(st.pop(0)) // 1
	fmt.Println(st.pop(1)) // 10
	fmt.Println(st.pop(2)) // 200
	fmt.Println(st.pop(2)) // 100
}
