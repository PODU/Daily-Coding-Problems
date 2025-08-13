// Day 112: LCA with parent pointers - equalize depths, walk up together. O(h).
package main

import "fmt"

type Node struct {
	Val            int
	Parent, L, R *Node
}

func mk(p *Node, v int) *Node { return &Node{Val: v, Parent: p} }

func depth(n *Node) int {
	d := 0
	for n != nil {
		n = n.Parent
		d++
	}
	return d
}

func lca(a, b *Node) *Node {
	da, db := depth(a), depth(b)
	for da > db {
		a = a.Parent
		da--
	}
	for db > da {
		b = b.Parent
		db--
	}
	for a != b {
		a = a.Parent
		b = b.Parent
	}
	return a
}

func main() {
	root := &Node{Val: 3}
	root.L = mk(root, 5)
	root.R = mk(root, 1)
	root.L.L = mk(root.L, 6)
	root.L.R = mk(root.L, 2)
	root.R.L = mk(root.R, 0)
	root.R.R = mk(root.R, 8)
	root.L.R.L = mk(root.L.R, 7)
	root.L.R.R = mk(root.L.R, 4)
	fmt.Println(lca(root.L, root.R).Val)         // 3
	fmt.Println(lca(root.L.R.L, root.L.R.R).Val) // 2
}
