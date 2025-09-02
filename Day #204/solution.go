// Day 204: Count nodes of a complete binary tree faster than O(n).
// Compare left/right spine heights: if equal subtree is perfect (2^h - 1), else recurse.
// Time: O(log^2 n), Space: O(log n).
package main

import "fmt"

type Node struct{ left, right *Node }

func leftHeight(n *Node) int  { h := 0; for n != nil { h++; n = n.left }; return h }
func rightHeight(n *Node) int { h := 0; for n != nil { h++; n = n.right }; return h }

func countNodes(root *Node) int {
	if root == nil {
		return 0
	}
	lh, rh := leftHeight(root), rightHeight(root)
	if lh == rh {
		return (1 << lh) - 1 // perfect subtree
	}
	return 1 + countNodes(root.left) + countNodes(root.right)
}

func main() {
	n := make([]*Node, 7)
	for i := 1; i < 7; i++ {
		n[i] = &Node{}
	}
	n[1].left, n[1].right = n[2], n[3]
	n[2].left, n[2].right = n[4], n[5]
	n[3].left = n[6]
	fmt.Println(countNodes(n[1])) // 6
}
