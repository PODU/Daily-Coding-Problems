// Postorder DFS: each node returns val+max(0,L,R) upward; global best = val+max(0,L)+max(0,R); O(n) time O(h) space
package main

import (
	"fmt"
	"math"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

var best int

func dfs(node *TreeNode) int {
	if node == nil {
		return 0
	}
	l := dfs(node.Left)
	if l < 0 {
		l = 0
	}
	r := dfs(node.Right)
	if r < 0 {
		r = 0
	}
	if node.Val+l+r > best {
		best = node.Val + l + r
	}
	if l > r {
		return node.Val + l
	}
	return node.Val + r
}

func maxPathSum(root *TreeNode) int {
	best = math.MinInt32
	dfs(root)
	return best
}

func main() {
	//       -10
	//       /  \
	//      9    20
	//          /  \
	//         15   7
	root1 := &TreeNode{Val: -10,
		Left: &TreeNode{Val: 9},
		Right: &TreeNode{Val: 20,
			Left:  &TreeNode{Val: 15},
			Right: &TreeNode{Val: 7},
		},
	}
	fmt.Printf("Max path sum: %d\n", maxPathSum(root1))

	//    1
	//   / \
	//  2   3
	root2 := &TreeNode{Val: 1,
		Left:  &TreeNode{Val: 2},
		Right: &TreeNode{Val: 3},
	}
	fmt.Printf("Max path sum: %d\n", maxPathSum(root2))
}
