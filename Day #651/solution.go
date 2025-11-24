// Validate BST with inclusive bounds: left<=node, right>=node (duplicates allowed both sides).
// Recursive (low,high) bound check. Time O(n), Space O(h). package main.
package main

import (
	"fmt"
	"math"
)

type TreeNode struct {
	val   int
	left  *TreeNode
	right *TreeNode
}

func isValid(node *TreeNode, low, high int) bool {
	if node == nil {
		return true
	}
	if node.val < low || node.val > high {
		return false
	}
	return isValid(node.left, low, node.val) &&
		isValid(node.right, node.val, high)
}

func isBST(root *TreeNode) bool {
	return isValid(root, math.MinInt, math.MaxInt)
}

func main() {
	// Tree A (valid): root=5, left=3(l=2,r=5), right=8(l=8,r=9)
	a := &TreeNode{val: 5}
	a.left = &TreeNode{val: 3}
	a.right = &TreeNode{val: 8}
	a.left.left = &TreeNode{val: 2}
	a.left.right = &TreeNode{val: 5}
	a.right.left = &TreeNode{val: 8}
	a.right.right = &TreeNode{val: 9}

	// Tree B (invalid): root=5, left=3, right=4
	b := &TreeNode{val: 5}
	b.left = &TreeNode{val: 3}
	b.right = &TreeNode{val: 4}

	fmt.Println(isBST(a))
	fmt.Println(isBST(b))
}
