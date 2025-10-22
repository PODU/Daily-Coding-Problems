// Tree locking with parent pointers + per-node lockedDescendantCount.
// lock/unlock are O(h): walk ancestors once to check, once to update counts.
package main

import "fmt"

type Node struct {
	name                  string
	parent, left, right   *Node
	locked                bool
	lockedDescendantCount int
}

func newNode(name string) *Node { return &Node{name: name} }

func (n *Node) isLocked() bool { return n.locked }

func (n *Node) anyAncestorLocked() bool {
	for p := n.parent; p != nil; p = p.parent {
		if p.locked {
			return true
		}
	}
	return false
}

func (n *Node) lock() bool {
	if n.locked {
		return false
	}
	if n.lockedDescendantCount > 0 { // a descendant is locked
		return false
	}
	if n.anyAncestorLocked() { // an ancestor is locked
		return false
	}
	n.locked = true
	for p := n.parent; p != nil; p = p.parent {
		p.lockedDescendantCount++
	}
	return true
}

func (n *Node) unlock() bool {
	if !n.locked {
		return false
	}
	n.locked = false
	for p := n.parent; p != nil; p = p.parent {
		p.lockedDescendantCount--
	}
	return true
}

func child(p, c *Node, left bool) *Node {
	if left {
		p.left = c
	} else {
		p.right = c
	}
	c.parent = p
	return c
}

func cap(b bool) string {
	if b {
		return "True"
	}
	return "False"
}

func main() {
	n1 := newNode("node1")
	n2 := newNode("node2")
	n3 := newNode("node3")
	n4 := newNode("node4")
	n5 := newNode("node5")
	child(n1, n2, true)
	child(n1, n3, false)
	child(n2, n4, true)
	child(n2, n5, false)

	fmt.Println("lock(node4): " + cap(n4.lock()))     // True
	fmt.Println("lock(node2): " + cap(n2.lock()))     // False (descendant locked)
	fmt.Println("unlock(node4): " + cap(n4.unlock())) // True
	fmt.Println("lock(node2): " + cap(n2.lock()))     // True
	fmt.Println("lock(node1): " + cap(n1.lock()))     // False (descendant locked)
	fmt.Println("lock(node5): " + cap(n5.lock()))     // False (ancestor locked)
	fmt.Println("unlock(node2): " + cap(n2.unlock())) // True
	fmt.Println("lock(node1): " + cap(n1.lock()))     // True
}
