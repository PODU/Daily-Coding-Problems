// Day 305: Remove consecutive nodes summing to zero. Prefix-sum + hashmap. O(N).
package main

import (
	"fmt"
	"strings"
)

type Node struct {
	val  int
	next *Node
}

func removeZeroSum(head *Node) *Node {
	dummy := &Node{0, head}
	seen := map[int]*Node{}
	prefix := 0
	for cur := dummy; cur != nil; cur = cur.next {
		prefix += cur.val
		seen[prefix] = cur
	}
	prefix = 0
	for cur := dummy; cur != nil; cur = cur.next {
		prefix += cur.val
		cur.next = seen[prefix].next
	}
	return dummy.next
}

func main() {
	var head, tail *Node
	for _, v := range []int{3, 4, -7, 5, -6, 6} {
		n := &Node{val: v}
		if head == nil {
			head, tail = n, n
		} else {
			tail.next = n
			tail = n
		}
	}
	head = removeZeroSum(head)
	var out []string
	for c := head; c != nil; c = c.next {
		out = append(out, fmt.Sprintf("%d", c.val))
	}
	fmt.Println(strings.Join(out, " ")) // 5
}
