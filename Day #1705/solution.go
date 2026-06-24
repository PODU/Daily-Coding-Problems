// Cousins: BFS tracking parent & depth; find target's depth+parent, collect nodes at
// same depth with different parent. Time O(n), Space O(n).
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
	level := []pair{{root, nil}}
	for len(level) > 0 {
		var targetParent *Node
		found := false
		var next []pair
		for _, p := range level {
			if p.node.Val == target {
				targetParent = p.parent
				found = true
			}
			if p.node.Left != nil {
				next = append(next, pair{p.node.Left, p.node})
			}
			if p.node.Right != nil {
				next = append(next, pair{p.node.Right, p.node})
			}
		}
		if found {
			res := []int{}
			for _, p := range level {
				if p.parent != targetParent {
					res = append(res, p.node.Val)
				}
			}
			return res
		}
		level = next
	}
	return []int{}
}

func main() {
	root := &Node{1,
		&Node{2, &Node{4, nil, nil}, &Node{5, nil, nil}},
		&Node{3, nil, &Node{6, nil, nil}}}
	fmt.Println(cousins(root, 4))
}
