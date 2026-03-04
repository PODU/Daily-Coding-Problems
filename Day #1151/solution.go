// Day 1151: Palindrome linked list (singly).
// Find middle via slow/fast, reverse 2nd half, compare. O(n) time, O(1) space.
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
	dummy := &Node{}
	t := dummy
	for _, x := range vals {
		t.next = &Node{val: x}
		t = t.next
	}
	return dummy.next
}

func main() {
	fmt.Println(boolStr(isPalindrome(build([]int{1, 4, 3, 4, 1})))) // True
	fmt.Println(boolStr(isPalindrome(build([]int{1, 4}))))          // False
}

func boolStr(b bool) string {
	if b {
		return "True"
	}
	return "False"
}
