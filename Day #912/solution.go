// Cousins: BFS level by level; on the target's level collect nodes whose parent differs. O(n) time, O(n) space.
package main

import "fmt"

type Node struct {
	Val         int
	Left, Right *Node
}

type pair struct {
	node, parent *Node
}

func cousins(root *Node, target int) []int {
	res := []int{}
	if root == nil {
		return res
	}
	queue := []pair{{root, nil}}
	for len(queue) > 0 {
		level := queue
		queue = nil
		var targetParent *Node
		found := false
		for _, p := range level {
			if p.node.Val == target {
				targetParent = p.parent
				found = true
			}
			if p.node.Left != nil {
				queue = append(queue, pair{p.node.Left, p.node})
			}
			if p.node.Right != nil {
				queue = append(queue, pair{p.node.Right, p.node})
			}
		}
		if found {
			for _, p := range level {
				if p.parent != targetParent && p.node.Val != target {
					res = append(res, p.node.Val)
				}
			}
			return res
		}
	}
	return res
}

func main() {
	root := &Node{Val: 1}
	root.Left = &Node{Val: 2}
	root.Right = &Node{Val: 3}
	root.Left.Left = &Node{Val: 4}
	root.Left.Right = &Node{Val: 5}
	root.Right.Right = &Node{Val: 6}

	fmt.Println("Cousins of 4:", cousins(root, 4)) // [6]
	fmt.Println("Cousins of 6:", cousins(root, 6)) // [4 5]
}
