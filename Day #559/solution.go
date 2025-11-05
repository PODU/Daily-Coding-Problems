// Merge k sorted linked lists using a min-heap (container/heap) of list heads.
// Time: O(N log k), Space: O(k) for the heap.
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

func buildList(vals []int) *ListNode {
	dummy := &ListNode{}
	cur := dummy
	for _, x := range vals {
		cur.Next = &ListNode{Val: x}
		cur = cur.Next
	}
	return dummy.Next
}

type NodeHeap []*ListNode

func (h NodeHeap) Len() int            { return len(h) }
func (h NodeHeap) Less(i, j int) bool  { return h[i].Val < h[j].Val }
func (h NodeHeap) Swap(i, j int)       { h[i], h[j] = h[j], h[i] }
func (h *NodeHeap) Push(x interface{}) { *h = append(*h, x.(*ListNode)) }
func (h *NodeHeap) Pop() interface{} {
	old := *h
	n := len(old)
	item := old[n-1]
	*h = old[:n-1]
	return item
}

func mergeKLists(lists []*ListNode) *ListNode {
	h := &NodeHeap{}
	heap.Init(h)
	for _, n := range lists {
		if n != nil {
			heap.Push(h, n)
		}
	}
	dummy := &ListNode{}
	tail := dummy
	for h.Len() > 0 {
		node := heap.Pop(h).(*ListNode)
		tail.Next = node
		tail = node
		if node.Next != nil {
			heap.Push(h, node.Next)
		}
	}
	return dummy.Next
}

func main() {
	lists := []*ListNode{
		buildList([]int{1, 4, 5}),
		buildList([]int{1, 3, 4}),
		buildList([]int{2, 6}),
	}
	merged := mergeKLists(lists)
	var parts []string
	for n := merged; n != nil; n = n.Next {
		parts = append(parts, fmt.Sprintf("%d", n.Val))
	}
	fmt.Println(strings.Join(parts, " "))
}
