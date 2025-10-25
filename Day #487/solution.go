// Day 487: Find all cousins of a target node (same level, different parent).
// BFS level by level tracking each node's parent; on the target's level collect nodes
// whose parent differs from the target's parent. Time O(n), Space O(n).
package main

import "fmt"

type Node struct {
	val         int
	left, right *Node
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
		var level []pair
		var targetParent *Node
		var next []pair
		for _, p := range queue {
			level = append(level, p)
			if p.node.val == target {
				targetParent = p.parent
			}
			if p.node.left != nil {
				next = append(next, pair{p.node.left, p.node})
			}
			if p.node.right != nil {
				next = append(next, pair{p.node.right, p.node})
			}
		}
		if targetParent != nil {
			for _, p := range level {
				if p.parent != targetParent && p.node.val != target {
					res = append(res, p.node.val)
				}
			}
			return res
		}
		queue = next
	}
	return res
}

func main() {
	root := &Node{1,
		&Node{2, &Node{4, nil, nil}, &Node{5, nil, nil}},
		&Node{3, nil, &Node{6, nil, nil}}}
	for _, v := range cousins(root, 4) {
		fmt.Println(v) // 6
	}
}
