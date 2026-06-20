// Two-pointer intersection of singly linked lists: redirect each pointer to the other head at end.
// Time O(M+N), Space O(1).
package main

import "fmt"

type Node struct {
	val  int
	next *Node
}

func getIntersection(headA, headB *Node) *Node {
	if headA == nil || headB == nil {
		return nil
	}
	pA, pB := headA, headB
	for pA != pB {
		if pA == nil {
			pA = headB
		} else {
			pA = pA.next
		}
		if pB == nil {
			pB = headA
		} else {
			pB = pB.next
		}
	}
	return pA
}

func main() {
	n8 := &Node{val: 8}
	n8.next = &Node{val: 10}
	a := &Node{val: 3}
	a.next = &Node{val: 7}
	a.next.next = n8
	b := &Node{val: 99}
	b.next = &Node{val: 1}
	b.next.next = n8

	res := getIntersection(a, b)
	if res != nil {
		fmt.Println(res.val)
	} else {
		fmt.Println("null")
	}
}
