// LCA with parent pointers: get depths via parent walk, level-up deeper node, advance both until equal. O(h) time O(1) space.
package main

import "fmt"

type Node struct {
	Val            int
	Left, Right, Parent *Node
}

func link(par, child *Node, isLeft bool) *Node {
	child.Parent = par
	if isLeft {
		par.Left = child
	} else {
		par.Right = child
	}
	return child
}

func nodeDepth(n *Node) int {
	d := 0
	for n.Parent != nil {
		n = n.Parent
		d++
	}
	return d
}

func lca(a, b *Node) *Node {
	da, db := nodeDepth(a), nodeDepth(b)
	for da > db { a = a.Parent; da-- }
	for db > da { b = b.Parent; db-- }
	for a != b { a = a.Parent; b = b.Parent }
	return a
}

func main() {
	n3 := &Node{Val: 3}
	n5 := link(n3, &Node{Val: 5}, true)
	n1 := link(n3, &Node{Val: 1}, false)
	n6 := link(n5, &Node{Val: 6}, true)
	n2 := link(n5, &Node{Val: 2}, false)
	link(n1, &Node{Val: 0}, true)
	link(n1, &Node{Val: 8}, false)
	n7 := link(n2, &Node{Val: 7}, true)
	n4 := link(n2, &Node{Val: 4}, false)

	fmt.Printf("LCA(7,4) = %d\n", lca(n7, n4).Val)
	fmt.Printf("LCA(6,4) = %d\n", lca(n6, n4).Val)
	fmt.Printf("LCA(7,1) = %d\n", lca(n7, n1).Val)
}
