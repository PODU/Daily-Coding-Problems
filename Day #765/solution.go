// Day 765: Remove the kth-from-last node in one pass with two pointers.
// fast leads slow by k; when fast hits the end, slow precedes the target.
// Time: O(n), Space: O(1).
package main

import (
	"fmt"
	"strconv"
	"strings"
)

type Node struct {
	val  int
	next *Node
}

func removeKthLast(head *Node, k int) *Node {
	dummy := &Node{next: head}
	fast, slow := dummy, dummy
	for i := 0; i < k; i++ {
		fast = fast.next
	}
	for fast.next != nil {
		fast = fast.next
		slow = slow.next
	}
	slow.next = slow.next.next // unlink target
	return dummy.next
}

func printList(head *Node) {
	parts := []string{}
	for p := head; p != nil; p = p.next {
		parts = append(parts, strconv.Itoa(p.val))
	}
	fmt.Println(strings.Join(parts, " -> "))
}

func main() {
	head := &Node{val: 1}
	cur := head
	for v := 2; v <= 5; v++ {
		cur.next = &Node{val: v}
		cur = cur.next
	}

	fmt.Print("before: ")
	printList(head)
	head = removeKthLast(head, 2)
	fmt.Print("after:  ")
	printList(head) // 1 -> 2 -> 3 -> 5
}
