// Day 256: Rearrange linked list values into zigzag low->high->low form.
// One pass over node values: at even i ensure a[i]<=a[i+1], at odd i ensure a[i]>=a[i+1], swap on violation.
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

func wiggle(head *Node) {
	less := true // even index: want current <= next
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

func listToString(head *Node) string {
	parts := []string{}
	for cur := head; cur != nil; cur = cur.next {
		parts = append(parts, strconv.Itoa(cur.val))
	}
	return strings.Join(parts, " -> ")
}

func main() {
	vals := []int{1, 2, 3, 4, 5}
	var head, tail *Node
	for _, v := range vals {
		n := &Node{val: v}
		if head == nil {
			head, tail = n, n
		} else {
			tail.next = n
			tail = n
		}
	}
	wiggle(head)
	fmt.Println(listToString(head)) // 1 -> 3 -> 2 -> 5 -> 4
}
