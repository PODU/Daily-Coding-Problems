// Binary tree locking: each node has a parent pointer and lockedDescendantCount.
// lock/unlock check ancestors (O(h)) + descendant count, then update ancestors (O(h)).
package main

import "fmt"

type Node struct {
	name                  string
	parent, left, right   *Node
	locked                bool
	lockedDescendantCount int
}

func NewNode(name string, parent *Node) *Node {
	return &Node{name: name, parent: parent}
}

func (n *Node) IsLocked() bool { return n.locked }

func (n *Node) canLock() bool {
	if n.lockedDescendantCount > 0 {
		return false
	}
	for cur := n.parent; cur != nil; cur = cur.parent {
		if cur.locked {
			return false
		}
	}
	return true
}

func (n *Node) Lock() bool {
	if n.locked || !n.canLock() {
		return false
	}
	n.locked = true
	for cur := n.parent; cur != nil; cur = cur.parent {
		cur.lockedDescendantCount++
	}
	return true
}

func (n *Node) Unlock() bool {
	if !n.locked {
		return false
	}
	n.locked = false
	for cur := n.parent; cur != nil; cur = cur.parent {
		cur.lockedDescendantCount--
	}
	return true
}

func main() {
	root := NewNode("root", nil)
	node1 := NewNode("node1", root)
	node2 := NewNode("node2", root)
	root.left, root.right = node1, node2
	node3 := NewNode("node3", node1)
	node4 := NewNode("node4", node1)
	node1.left, node1.right = node3, node4

	fmt.Printf("node2.lock() = %t\n", node2.Lock())
	fmt.Printf("root.lock() = %t\n", root.Lock())
	fmt.Printf("node2.unlock() = %t\n", node2.Unlock())
	fmt.Printf("root.lock() = %t\n", root.Lock())
}
