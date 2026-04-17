// Max path sum between any two nodes via DFS returning best downward gain.
// Time O(n), Space O(h).
package main

import (
	"fmt"
	"math"
)

type TreeNode struct {
	val         int
	left, right *TreeNode
}

func maxPathSum(root *TreeNode) int {
	best := math.MinInt64
	var gain func(n *TreeNode) int
	gain = func(n *TreeNode) int {
		if n == nil {
			return 0
		}
		l := gain(n.left)
		if l < 0 {
			l = 0
		}
		r := gain(n.right)
		if r < 0 {
			r = 0
		}
		if n.val+l+r > best {
			best = n.val + l + r
		}
		if l > r {
			return n.val + l
		}
		return n.val + r
	}
	gain(root)
	return best
}

func main() {
	root := &TreeNode{val: -10,
		left:  &TreeNode{val: 9},
		right: &TreeNode{val: 20, left: &TreeNode{val: 15}, right: &TreeNode{val: 7}}}
	fmt.Println(maxPathSum(root)) // 42
}
