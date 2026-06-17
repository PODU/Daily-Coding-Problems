// Day 1679: Size of largest BST subtree. Post-order returns (isBST, size, min, max)
// per subtree, tracking the global best. Time O(n), Space O(h).
package main

import (
	"fmt"
	"math"
)

type TreeNode struct {
	val         int
	left, right *TreeNode
}

type info struct {
	isBST    bool
	size     int
	mn, mx   float64
}

func largestBST(root *TreeNode) int {
	best := 0
	var dfs func(*TreeNode) info
	dfs = func(node *TreeNode) info {
		if node == nil {
			return info{true, 0, math.Inf(1), math.Inf(-1)}
		}
		l := dfs(node.left)
		r := dfs(node.right)
		v := float64(node.val)
		if l.isBST && r.isBST && l.mx < v && v < r.mn {
			sz := l.size + r.size + 1
			if sz > best {
				best = sz
			}
			return info{true, sz, math.Min(v, l.mn), math.Max(v, r.mx)}
		}
		return info{false, 0, math.Inf(-1), math.Inf(1)}
	}
	dfs(root)
	return best
}

func main() {
	root := &TreeNode{val: 10,
		left:  &TreeNode{val: 5, left: &TreeNode{val: 1}, right: &TreeNode{val: 8}},
		right: &TreeNode{val: 15, right: &TreeNode{val: 7}}}
	fmt.Println(largestBST(root)) // 3
}
