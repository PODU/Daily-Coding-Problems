// LCA with parent pointers: equalize depths, then walk both up together.
// Time O(h), Space O(1).
package main

import "fmt"

type Node struct {
	val               int
	parent, left, right *Node
}

func depth(n *Node) int {
	d := 0
	for n != nil {
		d++
		n = n.parent
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

func child(parent, c *Node) *Node {
	if c != nil {
		c.parent = parent
	}
	return c
}

func main() {
	n3, n5, n1 := &Node{val: 3}, &Node{val: 5}, &Node{val: 1}
	n6, n2, n0, n8 := &Node{val: 6}, &Node{val: 2}, &Node{val: 0}, &Node{val: 8}
	n7, n4 := &Node{val: 7}, &Node{val: 4}

	n3.left = child(n3, n5)
	n3.right = child(n3, n1)
	n5.left = child(n5, n6)
	n5.right = child(n5, n2)
	n1.left = child(n1, n0)
	n1.right = child(n1, n8)
	n2.left = child(n2, n7)
	n2.right = child(n2, n4)

	fmt.Println(lca(n6, n4).val)
	fmt.Println(lca(n6, n8).val)
}
