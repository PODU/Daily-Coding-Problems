// Day 715: Linked-list palindrome. Find middle (slow/fast), reverse second half,
// compare. Works for singly linked in O(n) time, O(1) space.
package main

import "fmt"

type Node struct {
	val  int
	next *Node
}

func isPalindrome(head *Node) bool {
	if head == nil || head.next == nil {
		return true
	}
	slow, fast := head, head
	for fast.next != nil && fast.next.next != nil {
		slow = slow.next
		fast = fast.next.next
	}
	var prev *Node
	cur := slow.next
	for cur != nil {
		nx := cur.next
		cur.next = prev
		prev = cur
		cur = nx
	}
	p1, p2 := head, prev
	for p2 != nil {
		if p1.val != p2.val {
			return false
		}
		p1 = p1.next
		p2 = p2.next
	}
	return true
}

func build(vals []int) *Node {
	var head *Node
	for i := len(vals) - 1; i >= 0; i-- {
		head = &Node{vals[i], head}
	}
	return head
}

func b2s(b bool) string {
	if b {
		return "True"
	}
	return "False"
}

func main() {
	fmt.Println(b2s(isPalindrome(build([]int{1, 4, 3, 4, 1}))))
	fmt.Println(b2s(isPalindrome(build([]int{1, 4}))))
}
