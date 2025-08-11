// Day 104: Linked-list palindrome (singly or doubly). Find middle, reverse second
// half, compare both halves. O(n) time, O(1) extra space.
package main

import "fmt"

type Node struct {
	val  int
	next *Node
}

func isPalindrome(head *Node) bool {
	slow, fast := head, head
	for fast != nil && fast.next != nil {
		slow = slow.next
		fast = fast.next.next
	}
	var prev *Node
	for slow != nil {
		nx := slow.next
		slow.next = prev
		prev = slow
		slow = nx
	}
	l, r := head, prev
	for r != nil {
		if l.val != r.val {
			return false
		}
		l, r = l.next, r.next
	}
	return true
}

func build(vals []int) *Node {
	var head *Node
	for i := len(vals) - 1; i >= 0; i-- {
		head = &Node{val: vals[i], next: head}
	}
	return head
}

func main() {
	fmt.Println(isPalindrome(build([]int{1, 4, 3, 4, 1}))) // true
	fmt.Println(isPalindrome(build([]int{1, 4})))          // false
}
