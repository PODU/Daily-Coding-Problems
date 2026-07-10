// Binary tree max path sum: DFS returning max downward gain; track global max = node + max(0,left) + max(0,right).
// Time O(n), Space O(h).
package main

import (
	"fmt"
	"math"
)

type Node struct {
	val         int
	left, right *Node
}

func maxPathSum(root *Node) int {
	best := math.MinInt64
	var gain func(n *Node) int
	gain = func(n *Node) int {
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
	root := &Node{val: -10,
		left:  &Node{val: 9},
		right: &Node{val: 20, left: &Node{val: 15}, right: &Node{val: 7}}}
	fmt.Println(maxPathSum(root)) // expected 42
}
