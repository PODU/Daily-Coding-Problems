// Largest BST subtree size via post-order DFS returning {isBST,size,min,max}. O(n) time.
package main

import (
	"fmt"
	"math"
)

type Node struct {
	val  int
	l, r *Node
}

type Info struct {
	isBST    bool
	size     int
	mn, mx   int
}

var best = 0

func dfs(n *Node) Info {
	if n == nil {
		return Info{true, 0, math.MaxInt32, math.MinInt32}
	}
	L := dfs(n.l)
	R := dfs(n.r)
	if L.isBST && R.isBST && n.val > L.mx && n.val < R.mn {
		sz := L.size + R.size + 1
		if sz > best {
			best = sz
		}
		mn := n.val
		if L.mn < mn {
			mn = L.mn
		}
		mx := n.val
		if R.mx > mx {
			mx = R.mx
		}
		return Info{true, sz, mn, mx}
	}
	return Info{false, 0, 0, 0}
}

func main() {
	root := &Node{val: 10}
	root.l = &Node{val: 5}
	root.r = &Node{val: 15}
	root.l.l = &Node{val: 1}
	root.l.r = &Node{val: 8}
	root.r.r = &Node{val: 7}
	dfs(root)
	fmt.Println(best)
}
