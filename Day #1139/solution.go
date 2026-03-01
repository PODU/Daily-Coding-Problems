// LCA via parent pointers: equalize depths then walk up together. O(h) time, O(1) space.
package main

import "fmt"

type Node struct {
	val                   int
	parent, left, right *Node
}

func depth(n *Node) int {
	d := 0
	for n.parent != nil {
		n = n.parent
		d++
	}
	return d
}

func lca(a, b *Node) *Node {
	da, db := depth(a), depth(b)
	for da > db {
		a = a.parent
		da--
	}
	for db > da {
		b = b.parent
		db--
	}
	for a != b {
		a = a.parent
		b = b.parent
	}
	return a
}

func link(p, c *Node, left bool) *Node {
	if left {
		p.left = c
	} else {
		p.right = c
	}
	c.parent = p
	return c
}

func main() {
	n1 := &Node{val: 1}
	n2, n3 := &Node{val: 2}, &Node{val: 3}
	n4, n5 := &Node{val: 4}, &Node{val: 5}
	n6, n7 := &Node{val: 6}, &Node{val: 7}
	link(n1, n2, true)
	link(n1, n3, false)
	link(n2, n4, true)
	link(n2, n5, false)
	link(n3, n6, true)
	link(n3, n7, false)

	fmt.Println(lca(n4, n5).val)
	fmt.Println(lca(n4, n6).val)
}
