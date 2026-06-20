// Zigzag list: single pass, even index wants lst[i]<=lst[i+1], odd wants lst[i]>=lst[i+1]; swap if violated.
// O(n) time, O(1) extra space.
package main

import (
	"fmt"
	"strconv"
	"strings"
)

type ListNode struct {
	val  int
	next *ListNode
}

func zigzag(head *ListNode) {
	cur := head
	i := 0
	for cur != nil && cur.next != nil {
		if i%2 == 0 {
			if cur.val > cur.next.val {
				cur.val, cur.next.val = cur.next.val, cur.val
			}
		} else {
			if cur.val < cur.next.val {
				cur.val, cur.next.val = cur.next.val, cur.val
			}
		}
		cur = cur.next
		i++
	}
}

func main() {
	head := &ListNode{val: 1}
	cur := head
	for _, v := range []int{2, 3, 4, 5} {
		cur.next = &ListNode{val: v}
		cur = cur.next
	}

	zigzag(head)

	var parts []string
	for cur = head; cur != nil; cur = cur.next {
		parts = append(parts, strconv.Itoa(cur.val))
	}
	fmt.Println(strings.Join(parts, " -> "))
}
