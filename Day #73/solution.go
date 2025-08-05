// Reverse singly linked list in place: iterate with prev/curr/next pointers. Time O(n), Space O(1).
package main

import (
	"fmt"
	"strconv"
	"strings"
)

type ListNode struct {
	Val  int
	Next *ListNode
}

func reverseList(head *ListNode) *ListNode {
	var prev *ListNode
	for head != nil {
		nxt := head.Next
		head.Next = prev
		prev = head
		head = nxt
	}
	return prev
}

func main() {
	head := &ListNode{Val: 1}
	head.Next = &ListNode{Val: 2}
	head.Next.Next = &ListNode{Val: 3}
	head.Next.Next.Next = &ListNode{Val: 4}
	head.Next.Next.Next.Next = &ListNode{Val: 5}

	head = reverseList(head)

	var vals []string
	for p := head; p != nil; p = p.Next {
		vals = append(vals, strconv.Itoa(p.Val))
	}
	fmt.Println(strings.Join(vals, " "))
}
