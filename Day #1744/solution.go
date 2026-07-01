// Merge k sorted linked lists via min-heap of current heads. O(N log k) time, O(k) space.
package main

import (
	"container/heap"
	"fmt"
	"strconv"
	"strings"
)

type Node struct {
	val  int
	next *Node
}

func build(vals []int) *Node {
	dummy := &Node{}
	t := dummy
	for _, x := range vals {
		t.next = &Node{val: x}
		t = t.next
	}
	return dummy.next
}

type NodeHeap []*Node

func (h NodeHeap) Len() int            { return len(h) }
func (h NodeHeap) Less(i, j int) bool  { return h[i].val < h[j].val }
func (h NodeHeap) Swap(i, j int)       { h[i], h[j] = h[j], h[i] }
func (h *NodeHeap) Push(x interface{}) { *h = append(*h, x.(*Node)) }
func (h *NodeHeap) Pop() interface{} {
	old := *h
	n := len(old)
	v := old[n-1]
	*h = old[:n-1]
	return v
}

func mergeK(lists []*Node) *Node {
	h := &NodeHeap{}
	heap.Init(h)
	for _, l := range lists {
		if l != nil {
			heap.Push(h, l)
		}
	}
	dummy := &Node{}
	tail := dummy
	for h.Len() > 0 {
		n := heap.Pop(h).(*Node)
		tail.next = n
		tail = n
		if n.next != nil {
			heap.Push(h, n.next)
		}
	}
	return dummy.next
}

func main() {
	lists := []*Node{build([]int{1, 4, 5}), build([]int{1, 3, 4}), build([]int{2, 6})}
	m := mergeK(lists)
	var out []string
	for p := m; p != nil; p = p.next {
		out = append(out, strconv.Itoa(p.val))
	}
	fmt.Println(strings.Join(out, " "))
}
