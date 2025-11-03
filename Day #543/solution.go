// Remove kth-from-last node in one pass via two pointers k apart. O(n) time, O(1) space.
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
	dummy := &Node{0, head}
	fast, slow := dummy, dummy
	for i := 0; i < k; i++ {
		fast = fast.next
	}
	for fast.next != nil {
		fast = fast.next
		slow = slow.next
	}
	slow.next = slow.next.next
	return dummy.next
}

func main() {
	head := &Node{1, &Node{2, &Node{3, &Node{4, &Node{5, nil}}}}}
	head = removeKthLast(head, 2)
	var parts []string
	for p := head; p != nil; p = p.next {
		parts = append(parts, strconv.Itoa(p.val))
	}
	fmt.Println(strings.Join(parts, " "))
}
