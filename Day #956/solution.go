// Day 956: merge k sorted singly linked lists using a min-heap.
// Time O(N log k), Space O(k) where N = total nodes.
package main

import (
	"container/heap"
	"fmt"
	"strings"
)

type ListNode struct {
	val  int
	next *ListNode
}

type nodeHeap []*ListNode

func (h nodeHeap) Len() int            { return len(h) }
func (h nodeHeap) Less(i, j int) bool  { return h[i].val < h[j].val }
func (h nodeHeap) Swap(i, j int)       { h[i], h[j] = h[j], h[i] }
func (h *nodeHeap) Push(x interface{}) { *h = append(*h, x.(*ListNode)) }
func (h *nodeHeap) Pop() interface{} {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[:n-1]
	return x
}

func mergeK(lists []*ListNode) *ListNode {
	h := &nodeHeap{}
	for _, l := range lists {
		if l != nil {
			heap.Push(h, l)
		}
	}
	dummy := &ListNode{}
	tail := dummy
	for h.Len() > 0 {
		n := heap.Pop(h).(*ListNode)
		tail.next = n
		tail = n
		if n.next != nil {
			heap.Push(h, n.next)
		}
	}
	return dummy.next
}

func build(vals []int) *ListNode {
	dummy := &ListNode{}
	t := dummy
	for _, x := range vals {
		t.next = &ListNode{val: x}
		t = t.next
	}
	return dummy.next
}

func main() {
	lists := []*ListNode{build([]int{1, 4, 5}), build([]int{1, 3, 4}), build([]int{2, 6})}
	m := mergeK(lists)
	var parts []string
	for p := m; p != nil; p = p.next {
		parts = append(parts, fmt.Sprintf("%d", p.val))
	}
	fmt.Println(strings.Join(parts, " ")) // 1 1 2 3 4 4 5 6
}
