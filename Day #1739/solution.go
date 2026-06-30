// Tree node with parent pointer + lockedDescendants count; lock/unlock walk ancestors O(h).
// IsLocked O(1). Lock/Unlock O(h). Space O(n).
package main

import "fmt"

type Node struct {
	parent            *Node
	left              *Node
	right             *Node
	locked            bool
	lockedDescendants int
}

func NewNode(parent *Node) *Node { return &Node{parent: parent} }

func (n *Node) IsLocked() bool { return n.locked }

func (n *Node) anyAncestorLocked() bool {
	for p := n.parent; p != nil; p = p.parent {
		if p.locked {
			return true
		}
	}
	return false
}

func (n *Node) Lock() bool {
	if n.locked || n.lockedDescendants > 0 || n.anyAncestorLocked() {
		return false
	}
	n.locked = true
	for p := n.parent; p != nil; p = p.parent {
		p.lockedDescendants++
	}
	return true
}

func (n *Node) Unlock() bool {
	if !n.locked || n.lockedDescendants > 0 || n.anyAncestorLocked() {
		return false
	}
	n.locked = false
	for p := n.parent; p != nil; p = p.parent {
		p.lockedDescendants--
	}
	return true
}

func main() {
	root := NewNode(nil)
	root.left = NewNode(root)
	root.right = NewNode(root)
	root.left.left = NewNode(root.left)
	root.left.right = NewNode(root.left)
	L := root.left
	LL := root.left.left

	fmt.Println(LL.Lock())
	fmt.Println(L.Lock())
	fmt.Println(root.Lock())
	fmt.Println(LL.Unlock())
	fmt.Println(L.Lock())
	fmt.Println(root.Lock())
}
