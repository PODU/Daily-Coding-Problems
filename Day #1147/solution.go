// Day 1147: Locking in a binary tree.
// Node keeps parent + count of locked descendants; lock/unlock walk ancestors. O(h).
package main

import "fmt"

type Node struct {
	left, right, parent *Node
	locked              bool
	lockedDesc          int
}

func (n *Node) IsLocked() bool { return n.locked }

func (n *Node) canLock() bool {
	if n.locked || n.lockedDesc > 0 {
		return false
	}
	for a := n.parent; a != nil; a = a.parent {
		if a.locked {
			return false
		}
	}
	return true
}

func (n *Node) Lock() bool {
	if !n.canLock() {
		return false
	}
	n.locked = true
	for a := n.parent; a != nil; a = a.parent {
		a.lockedDesc++
	}
	return true
}

func (n *Node) Unlock() bool {
	if !n.locked {
		return false
	}
	n.locked = false
	for a := n.parent; a != nil; a = a.parent {
		a.lockedDesc--
	}
	return true
}

func main() {
	root := &Node{}
	l := &Node{parent: root}
	r := &Node{parent: root}
	ll := &Node{parent: l}
	root.left, root.right, l.left = l, r, ll
	fmt.Println(l.Lock())    // true
	fmt.Println(ll.Lock())   // false
	fmt.Println(root.Lock()) // false
	fmt.Println(l.Unlock())  // true
	fmt.Println(ll.Lock())   // true
}
