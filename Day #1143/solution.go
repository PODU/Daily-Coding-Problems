// Day 1143: Merge k sorted linked lists.
// Min-heap of list heads. Time O(N log k), Space O(k).
package main

import (
	"container/heap"
	"fmt"
	"strings"
)

type Node struct {
	val  int
	next *Node
}

type NodeHeap []*Node

func (h NodeHeap) Len() int            { return len(h) }
func (h NodeHeap) Less(i, j int) bool  { return h[i].val < h[j].val }
func (h NodeHeap) Swap(i, j int)       { h[i], h[j] = h[j], h[i] }
func (h *NodeHeap) Push(x interface{}) { *h = append(*h, x.(*Node)) }
func (h *NodeHeap) Pop() interface{} {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[:n-1]
	return x
}

func mergeK(lists []*Node) *Node {
	h := &NodeHeap{}
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

func build(vals []int) *Node {
	dummy := &Node{}
	t := dummy
	for _, x := range vals {
		t.next = &Node{val: x}
		t = t.next
	}
	return dummy.next
}

func main() {
	lists := []*Node{build([]int{1, 4, 7}), build([]int{2, 5, 8}), build([]int{3, 6, 9})}
	var out []string
	for n := mergeK(lists); n != nil; n = n.next {
		out = append(out, fmt.Sprintf("%d", n.val))
	}
	fmt.Println(strings.Join(out, " -> ")) // 1 -> 2 -> ... -> 9
}
