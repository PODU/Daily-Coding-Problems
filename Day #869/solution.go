// Day 869: Is a linked list a palindrome (works for singly linked; doubly trivial).
// Approach: find middle (slow/fast), reverse second half, compare both halves. O(1) space.
// Time: O(n), Space: O(1).
package main

import "fmt"

type Node struct {
	val  int
	next *Node
}

func build(vals []int) *Node {
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
