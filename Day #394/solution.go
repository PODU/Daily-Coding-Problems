// Root-to-leaf path sum via DFS subtracting node values; leaf checks remainder==0. O(n) time, O(h) space.
package main

import "fmt"

type Node struct {
	val         int
	left, right *Node
}

func hasPathSum(root *Node, k int) bool {
	if root == nil {
		return false
	}
	if root.left == nil && root.right == nil {
		return k-root.val == 0
	}
	return hasPathSum(root.left, k-root.val) || hasPathSum(root.right, k-root.val)
}

func main() {
	root := &Node{val: 8,
		left:  &Node{val: 4, left: &Node{val: 2}, right: &Node{val: 6}},
		right: &Node{val: 13, right: &Node{val: 19}}}
	fmt.Println(hasPathSum(root, 18))
}
