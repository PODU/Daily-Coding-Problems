// Zigzag rearrange linked list values in a single pass by swapping adjacent
// node values that violate the expected ordering. Time O(n), Space O(1).
package main

import (
	"fmt"
	"strings"
)

type Node struct {
	val  int
	next *Node
}

func zigzag(head *Node) {
	less := true // even index expects list[i] <= list[i+1]
	for cur := head; cur != nil && cur.next != nil; cur = cur.next {
		if less {
			if cur.val > cur.next.val {
				cur.val, cur.next.val = cur.next.val, cur.val
			}
		} else {
			if cur.val < cur.next.val {
				cur.val, cur.next.val = cur.next.val, cur.val
			}
		}
		less = !less
	}
}

func main() {
	var head, tail *Node
	for _, v := range []int{1, 2, 3, 4, 5} {
		n := &Node{val: v}
		if head == nil {
			head, tail = n, n
		} else {
			tail.next = n
			tail = n
		}
	}
	zigzag(head)
	var parts []string
	for cur := head; cur != nil; cur = cur.next {
		parts = append(parts, fmt.Sprintf("%d", cur.val))
	}
	fmt.Println(strings.Join(parts, " -> "))
}
