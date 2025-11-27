// Day 664: Maximum path sum between any two nodes in a binary tree.
// Post-order DFS: each node returns best downward gain; track best "bridge". Time O(n), Space O(h).
package main

import (
	"fmt"
	"math"
)

type Node struct {
	val  int
	l, r *Node
}

func maxPathSum(root *Node) int {
	best := math.MinInt64
	var gain func(n *Node) int
	gain = func(n *Node) int {
		if n == nil {
			return 0
		}
		lg := gain(n.l)
		if lg < 0 {
			lg = 0
		}
		rg := gain(n.r)
		if rg < 0 {
			rg = 0
		}
		if n.val+lg+rg > best {
			best = n.val + lg + rg
		}
		if lg > rg {
			return n.val + lg
		}
		return n.val + rg
	}
	gain(root)
	return best
}

func main() {
	root := &Node{val: -10,
		l: &Node{val: 9},
		r: &Node{val: 20, l: &Node{val: 15}, r: &Node{val: 7}}}
	fmt.Println(maxPathSum(root)) // 42
}
