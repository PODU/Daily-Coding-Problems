// Day 417: Remove all consecutive nodes summing to zero via prefix-sum + hashmap.
// Time O(n), Space O(n).
package main

import (
	"fmt"
	"strings"
)

type ListNode struct {
	Val  int
	Next *ListNode
}

func removeZeroSum(head *ListNode) *ListNode {
	dummy := &ListNode{Next: head}
	seen := make(map[int]*ListNode)
	prefix := 0
	for node := dummy; node != nil; node = node.Next {
		prefix += node.Val
		seen[prefix] = node // keep latest node for this prefix sum
	}
	prefix = 0
	for node := dummy; node != nil; node = node.Next {
		prefix += node.Val
		node.Next = seen[prefix].Next // skip zero-sum run
	}
	return dummy.Next
}

func build(vals []int) *ListNode {
	dummy := &ListNode{}
	tail := dummy
	for _, v := range vals {
		tail.Next = &ListNode{Val: v}
		tail = tail.Next
	}
	return dummy.Next
}

func main() {
	head := removeZeroSum(build([]int{3, 4, -7, 5, -6, 6}))
	var parts []string
	for n := head; n != nil; n = n.Next {
		parts = append(parts, fmt.Sprintf("%d", n.Val))
	}
	fmt.Println(strings.Join(parts, " -> "))
}
