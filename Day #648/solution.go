// Deep clone linked list with random pointers using O(1) interleaving (3 passes).
// Time O(n), Space O(1) extra. package main.
package main

import "fmt"

type Node struct {
	val    int
	next   *Node
	random *Node
}

func cloneList(head *Node) *Node {
	if head == nil {
		return nil
	}
	// Pass 1: insert cloned node after each original
	for cur := head; cur != nil; cur = cur.next.next {
		copy := &Node{val: cur.val}
		copy.next = cur.next
		cur.next = copy
	}
	// Pass 2: set clone.random
	for cur := head; cur != nil; cur = cur.next.next {
		if cur.random != nil {
			cur.next.random = cur.random.next
		} else {
			cur.next.random = nil
		}
	}
	// Pass 3: split lists
	newHead := head.next
	for cur := head; cur != nil; cur = cur.next {
		copy := cur.next
		cur.next = copy.next
		if copy.next != nil {
			copy.next = copy.next.next
		} else {
			copy.next = nil
		}
	}
	return newHead
}

func main() {
	n1, n2, n3, n4 := &Node{val: 1}, &Node{val: 2}, &Node{val: 3}, &Node{val: 4}
	n1.next = n2
	n2.next = n3
	n3.next = n4
	n1.random = n3
	n2.random = n1
	n3.random = n3
	n4.random = n2

	cloned := cloneList(n1)

	origSet := make(map[*Node]bool)
	for cur := n1; cur != nil; cur = cur.next {
		origSet[cur] = true
	}

	deep := true
	for cur := cloned; cur != nil; cur = cur.next {
		rv := 0
		if cur.random != nil {
			rv = cur.random.val
		}
		fmt.Printf("node %d random %d\n", cur.val, rv)
		if origSet[cur] {
			deep = false
		}
		if cur.random != nil && origSet[cur.random] {
			deep = false
		}
	}
	fmt.Printf("deep copy verified: %t\n", deep)
}
