// Day 540: Boustrophedon (zigzag) level-order traversal of a binary tree.
// BFS level by level, reversing every other level. Time O(n), Space O(n).
package main

import "fmt"

type Node struct {
	Val         int
	Left, Right *Node
}

func boustrophedon(root *Node) []int {
	out := []int{}
	if root == nil {
		return out
	}
	queue := []*Node{root}
	leftToRight := true
	for len(queue) > 0 {
		var next []*Node
		level := make([]int, len(queue))
		for i, n := range queue {
			idx := i
			if !leftToRight {
				idx = len(queue) - 1 - i
			}
			level[idx] = n.Val
			if n.Left != nil {
				next = append(next, n.Left)
			}
			if n.Right != nil {
				next = append(next, n.Right)
			}
		}
		out = append(out, level...)
		leftToRight = !leftToRight
		queue = next
	}
	return out
}

func main() {
	root := &Node{1,
		&Node{2, &Node{4, nil, nil}, &Node{5, nil, nil}},
		&Node{3, &Node{6, nil, nil}, &Node{7, nil, nil}}}
	fmt.Println(boustrophedon(root))
}
