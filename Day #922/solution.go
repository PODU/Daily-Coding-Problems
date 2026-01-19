// Locking binary tree: each node tracks lockedDescendants count; lock/unlock check
// ancestors + descendant count. All ops O(h) where h is tree height.
package main

import "fmt"

type Node struct {
	parent            *Node
	left              *Node
	right             *Node
	locked            bool
	lockedDescendants int
}

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
	if !n.locked {
		return false
	}
	n.locked = false
	for p := n.parent; p != nil; p = p.parent {
		p.lockedDescendants--
	}
	return true
}

func main() {
	root, a, b, c, d := &Node{}, &Node{}, &Node{}, &Node{}, &Node{}
	root.left, root.right = a, b
	a.parent, b.parent = root, root
	a.left, a.right = c, d
	c.parent, d.parent = a, a

	fmt.Printf("lock c (leaf)      -> %v (expect true)\n", c.Lock())
	fmt.Printf("lock a (ancestor)  -> %v (expect false)\n", a.Lock())
	fmt.Printf("lock root          -> %v (expect false)\n", root.Lock())
	fmt.Printf("unlock c           -> %v (expect true)\n", c.Unlock())
	fmt.Printf("lock a             -> %v (expect true)\n", a.Lock())
	fmt.Printf("lock c (desc lock) -> %v (expect false)\n", c.Lock())
	fmt.Printf("unlock a           -> %v (expect true)\n", a.Unlock())
}
