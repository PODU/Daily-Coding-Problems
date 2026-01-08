// Largest BST subtree via post-order returning (isBST, size, min, max). Time O(n), Space O(h).
package main

import (
	"fmt"
	"math"
)

type Node struct {
	val         int
	left, right *Node
}

type info struct {
	isBST   bool
	size    int
	mn, mx  int
}

func largestBST(root *Node) int {
	best := 0
	var dfs func(n *Node) info
	dfs = func(n *Node) info {
		if n == nil {
			return info{true, 0, math.MaxInt32, math.MinInt32}
		}
		l := dfs(n.left)
		r := dfs(n.right)
		if l.isBST && r.isBST && l.mx < n.val && n.val < r.mn {
			sz := l.size + r.size + 1
			if sz > best {
				best = sz
			}
			mn := n.val
			if l.mn < mn {
				mn = l.mn
			}
			mx := n.val
			if r.mx > mx {
				mx = r.mx
			}
			return info{true, sz, mn, mx}
		}
		return info{false, 0, 0, 0}
	}
	dfs(root)
	return best
}

func main() {
	root := &Node{10,
		&Node{5, &Node{1, nil, nil}, &Node{8, nil, nil}},
		&Node{15, nil, &Node{7, nil, nil}}}
	fmt.Println(largestBST(root))
}
