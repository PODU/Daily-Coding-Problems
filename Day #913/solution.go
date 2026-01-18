// Complete-tree node count: if left height == right height subtree is perfect (2^h-1), else recurse. O(log^2 n).
package main

import "fmt"

type Node struct {
	Val         int
	Left, Right *Node
}

func leftHeight(n *Node) int  { h := 0; for n != nil { h++; n = n.Left }; return h }
func rightHeight(n *Node) int { h := 0; for n != nil { h++; n = n.Right }; return h }

func countNodes(root *Node) int {
	if root == nil {
		return 0
	}
	lh, rh := leftHeight(root), rightHeight(root)
	if lh == rh {
		return (1 << lh) - 1 // perfect subtree
	}
	return 1 + countNodes(root.Left) + countNodes(root.Right)
}

func main() {
	// Complete tree with 6 nodes (values 1..6 in level order):
	//         1
	//       /   \
	//      2     3
	//     / \   /
	//    4   5 6
	n := make([]*Node, 7)
	for v := 1; v <= 6; v++ {
		n[v] = &Node{Val: v}
	}
	n[1].Left, n[1].Right = n[2], n[3]
	n[2].Left, n[2].Right = n[4], n[5]
	n[3].Left = n[6]

	fmt.Println(countNodes(n[1])) // 6
}
