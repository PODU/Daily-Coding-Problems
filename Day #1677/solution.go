// Day 1677: Linked-list palindrome. Singly: find middle, reverse 2nd half, compare.
// Doubly: two pointers from both ends. Time O(n), Space O(1).
package main

import "fmt"

type Node struct {
	val        int
	next, prev *Node
}

func build(values []int) *Node {
	var head, tail *Node
	for _, x := range values {
		n := &Node{val: x}
		if head == nil {
			head, tail = n, n
		} else {
			tail.next = n
			n.prev = tail
			tail = n
		}
	}
	return head
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
	a, b := head, prev
	for b != nil {
		if a.val != b.val {
			return false
		}
		a, b = a.next, b.next
	}
	return true
}

func main() {
	fmt.Println(isPalindrome(build([]int{1, 4, 3, 4, 1}))) // true
	fmt.Println(isPalindrome(build([]int{1, 4})))          // false
}
