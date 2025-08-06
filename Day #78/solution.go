// Day 78: Merge k sorted linked lists using a min-heap.
// Time O(N log k) where N total nodes, Space O(k).
package main

import (
	"container/heap"
	"fmt"
	"strings"
)

type ListNode struct {
	Val  int
	Next *ListNode
}

type NodeHeap []*ListNode

func (h NodeHeap) Len() int            { return len(h) }
func (h NodeHeap) Less(i, j int) bool  { return h[i].Val < h[j].Val }
func (h NodeHeap) Swap(i, j int)       { h[i], h[j] = h[j], h[i] }
func (h *NodeHeap) Push(x interface{}) { *h = append(*h, x.(*ListNode)) }
func (h *NodeHeap) Pop() interface{} {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[:n-1]
	return x
}

func mergeKLists(lists []*ListNode) *ListNode {
	h := &NodeHeap{}
	heap.Init(h)
	for _, l := range lists {
		if l != nil {
			heap.Push(h, l)
		}
	}
	dummy := &ListNode{}
	tail := dummy
	for h.Len() > 0 {
		n := heap.Pop(h).(*ListNode)
		tail.Next = n
		tail = n
		if n.Next != nil {
			heap.Push(h, n.Next)
		}
	}
	return dummy.Next
}

func build(vals []int) *ListNode {
	dummy := &ListNode{}
	t := dummy
	for _, x := range vals {
		t.Next = &ListNode{Val: x}
		t = t.Next
	}
	return dummy.Next
}

func main() {
	lists := []*ListNode{build([]int{1, 4, 5}), build([]int{1, 3, 4}), build([]int{2, 6})}
	m := mergeKLists(lists)
	parts := []string{}
	for ; m != nil; m = m.Next {
		parts = append(parts, fmt.Sprintf("%d", m.Val))
	}
	fmt.Println(strings.Join(parts, " -> ")) // 1 -> 1 -> 2 -> 3 -> 4 -> 4 -> 5 -> 6
}
